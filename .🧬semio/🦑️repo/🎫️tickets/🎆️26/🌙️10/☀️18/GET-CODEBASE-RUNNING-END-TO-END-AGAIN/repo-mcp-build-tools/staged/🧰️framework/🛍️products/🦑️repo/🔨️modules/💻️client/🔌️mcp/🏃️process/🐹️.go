// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// The child process port of the build surface: starts the `repo` executable, captures its output within bounds, turns its typed progress lines into events and stops it when the call is cancelled.

// #endregion 🧲️Header

package main

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"os"
	"os/exec"
	"strings"
	"sync"
	"time"
	"unicode/utf8"
)

// #region 📈️Progress

// 📈️ScriptProgress is the typed line a child prints on stderr as `[script] <json>` (see `$defs.ScriptProgress` of the module schema).
type ScriptProgress struct {
	Owner    string          `json:"owner"`
	Command  string          `json:"command"`
	Stage    string          `json:"stage"`
	Activity *ScriptActivity `json:"activity,omitempty"`
}

type ScriptActivity struct {
	Resource  string  `json:"resource"`
	Mode      string  `json:"mode"`
	ElapsedMs float64 `json:"elapsedMs"`
}

const scriptPrefix = "[script] "

// 📈️ParseScriptLine recognises a progress line: a typed record, a human line that explains the previous record, or neither.
func ParseScriptLine(line string) (record *ScriptProgress, human string, ok bool) {
	if !strings.HasPrefix(line, scriptPrefix) {
		return nil, "", false
	}
	body := strings.TrimSpace(strings.TrimPrefix(line, scriptPrefix))
	if strings.HasPrefix(body, "{") {
		var parsed ScriptProgress
		if err := json.Unmarshal([]byte(body), &parsed); err == nil && parsed.Stage != "" {
			return &parsed, "", true
		}
	}
	return nil, body, true
}

// 📈️Describe renders a typed record in one line when the child printed no human line for it.
func (progress ScriptProgress) Describe() string {
	text := progress.Command + " " + progress.Stage
	if progress.Activity != nil {
		text += ": " + progress.Activity.Resource + " (" + progress.Activity.Mode + ")"
	}
	return text
}

// #endregion 📈️Progress

// #region 📦️Capture

// 📦️Capture keeps the first bytes for parsing and the last bytes for reading, and counts the rest.
type Capture struct {
	mu       sync.Mutex
	head     []byte
	tail     []byte
	headMax  int
	tailMax  int
	total    int64
	overflow bool
	last     string
}

func NewCapture(headMax, tailMax int) *Capture {
	return &Capture{headMax: headMax, tailMax: tailMax}
}

func (capture *Capture) Write(data []byte) (int, error) {
	capture.mu.Lock()
	defer capture.mu.Unlock()
	capture.total += int64(len(data))
	if room := capture.headMax - len(capture.head); room > 0 {
		capture.head = append(capture.head, data[:min(room, len(data))]...)
	}
	if len(capture.head) >= capture.headMax && capture.total > int64(capture.headMax) {
		capture.overflow = true
	}
	capture.tail = append(capture.tail, data...)
	if len(capture.tail) > 2*capture.tailMax {
		capture.tail = append([]byte(nil), capture.tail[len(capture.tail)-capture.tailMax:]...)
	}
	if trimmed := bytes.TrimRight(data, "\r\n"); len(trimmed) > 0 {
		line := trimmed[bytes.LastIndexByte(trimmed, '\n')+1:]
		if text := strings.TrimSpace(string(line)); text != "" {
			capture.last = text[:min(len(text), 200)]
		}
	}
	return len(data), nil
}

// 📦️Captured is a snapshot of a capture.
type Captured struct {
	Head      []byte
	Tail      []byte
	Total     int64
	Truncated bool
	Overflow  bool
}

func (capture *Capture) Snapshot() Captured {
	capture.mu.Lock()
	defer capture.mu.Unlock()
	tail := capture.tail
	truncated := capture.total > int64(len(tail))
	if len(tail) > capture.tailMax {
		tail, truncated = tail[len(tail)-capture.tailMax:], true
	}
	return Captured{Head: append([]byte(nil), capture.head...), Tail: append([]byte(nil), tail...), Total: capture.total, Truncated: truncated, Overflow: capture.overflow}
}

// 🔚️LastLine is the most recent non-empty line.
func (capture *Capture) LastLine() string {
	capture.mu.Lock()
	defer capture.mu.Unlock()
	return capture.last
}

// 🧼️Printable drops terminal escape sequences and invalid UTF-8 so captured output is safe to show.
func Printable(data []byte) string {
	var out bytes.Buffer
	for index := 0; index < len(data); {
		if data[index] == 0x1b && index+1 < len(data) && data[index+1] == '[' {
			index += 2
			for index < len(data) && !(data[index] >= 0x40 && data[index] <= 0x7e) {
				index++
			}
			index++
			continue
		}
		if data[index] == 0x1b && index+1 < len(data) && data[index+1] == ']' {
			index += 2
			for index < len(data) && data[index] != 0x07 && data[index] != 0x1b {
				index++
			}
			index++
			continue
		}
		out.WriteByte(data[index])
		index++
	}
	text := out.String()
	if !utf8.ValidString(text) {
		text = strings.ToValidUTF8(text, "�")
	}
	return strings.ReplaceAll(text, "\x00", "")
}

// #endregion 📦️Capture

// #region 🏃️Runner

// 🏃️ProcessSpec is one child to run.
type ProcessSpec struct {
	Binary    string
	Prefix    []string
	Argv      []string
	Dir       string
	Env       []string
	Grace     time.Duration
	Heartbeat time.Duration
	HeadMax   int
	StdoutMax int
	StderrMax int
}

// 🏃️ProcessEvent is something the call can report: a typed progress record, its human line, new output, or a heartbeat.
type ProcessEvent struct {
	Kind    string
	Script  *ScriptProgress
	Message string
}

// 🏃️ProcessOutcome is how a child ended.
type ProcessOutcome struct {
	Started        bool
	StartError     string
	ExitCode       int
	Signal         string
	Cancelled      bool
	Elapsed        time.Duration
	ProgressEvents int
	Stdout         Captured
	Stderr         Captured
}

// 🏃️ProcessRunner starts a child and waits for it. Cancelling the context interrupts the child, then kills it after the grace period.
type ProcessRunner interface {
	Run(ctx context.Context, spec ProcessSpec, emit func(ProcessEvent)) ProcessOutcome
}

// 🏃️ExecRunner runs children with the operating system.
type ExecRunner struct{}

// stderrSink splits stderr into progress lines and captured text.
type stderrSink struct {
	mu      sync.Mutex
	pending []byte
	capture *Capture
	emit    func(ProcessEvent)
	events  *int
	waiting *ScriptProgress
}

func (sink *stderrSink) Write(data []byte) (int, error) {
	sink.mu.Lock()
	defer sink.mu.Unlock()
	sink.pending = append(sink.pending, data...)
	for {
		newline := bytes.IndexByte(sink.pending, '\n')
		if newline < 0 {
			if len(sink.pending) > 1<<20 {
				sink.line(string(sink.pending))
				sink.pending = nil
			}
			break
		}
		sink.line(strings.TrimRight(string(sink.pending[:newline]), "\r"))
		sink.pending = sink.pending[newline+1:]
	}
	return len(data), nil
}

func (sink *stderrSink) line(text string) {
	record, human, isScript := ParseScriptLine(text)
	switch {
	case !isScript:
		sink.flush()
		_, _ = sink.capture.Write([]byte(text + "\n"))
	case record != nil:
		sink.flush()
		sink.waiting = record
	default:
		if sink.waiting != nil {
			*sink.events++
			sink.emit(ProcessEvent{Kind: "script", Script: sink.waiting, Message: human})
			sink.waiting = nil
		} else {
			*sink.events++
			sink.emit(ProcessEvent{Kind: "script", Message: human})
		}
	}
}

// flush reports a typed record that no human line followed.
func (sink *stderrSink) flush() {
	if sink.waiting != nil {
		*sink.events++
		sink.emit(ProcessEvent{Kind: "script", Script: sink.waiting, Message: sink.waiting.Describe()})
		sink.waiting = nil
	}
}

func (sink *stderrSink) finish() {
	sink.mu.Lock()
	defer sink.mu.Unlock()
	if len(sink.pending) > 0 {
		sink.line(strings.TrimRight(string(sink.pending), "\r\n"))
		sink.pending = nil
	}
	sink.flush()
}

func (ExecRunner) Run(ctx context.Context, spec ProcessSpec, emit func(ProcessEvent)) ProcessOutcome {
	started := time.Now()
	outcome := ProcessOutcome{ExitCode: -1}
	stdout := NewCapture(spec.HeadMax, spec.StdoutMax)
	stderr := NewCapture(spec.HeadMax, spec.StderrMax)
	events := 0
	var emitMu sync.Mutex
	safeEmit := func(event ProcessEvent) {
		emitMu.Lock()
		defer emitMu.Unlock()
		emit(event)
	}
	sink := &stderrSink{capture: stderr, emit: safeEmit, events: &events}
	command := exec.Command(spec.Binary, append(append([]string(nil), spec.Prefix...), spec.Argv...)...)
	command.Dir, command.Env = spec.Dir, spec.Env
	command.Stdin = nil
	command.Stdout, command.Stderr = stdout, sink
	command.WaitDelay = spec.Grace
	if err := command.Start(); err != nil {
		outcome.StartError = err.Error()
		outcome.Elapsed = time.Since(started)
		return outcome
	}
	outcome.Started = true
	finished := make(chan struct{})
	var watchers sync.WaitGroup
	watchers.Add(2)
	go func() {
		defer watchers.Done()
		select {
		case <-ctx.Done():
			outcome.Cancelled = true
			if err := command.Process.Signal(os.Interrupt); err != nil {
				_ = command.Process.Kill()
				return
			}
			timer := time.NewTimer(spec.Grace)
			defer timer.Stop()
			select {
			case <-finished:
			case <-timer.C:
				_ = command.Process.Kill()
			}
		case <-finished:
		}
	}()
	go func() {
		defer watchers.Done()
		if spec.Heartbeat <= 0 {
			return
		}
		ticker := time.NewTicker(spec.Heartbeat)
		defer ticker.Stop()
		for {
			select {
			case <-ticker.C:
				message := stdout.LastLine()
				if message == "" {
					message = stderr.LastLine()
				}
				safeEmit(ProcessEvent{Kind: "heartbeat", Message: message})
			case <-finished:
				return
			}
		}
	}()
	waitError := command.Wait()
	close(finished)
	watchers.Wait()
	sink.finish()
	outcome.Elapsed = time.Since(started)
	outcome.ProgressEvents = events
	outcome.Stdout, outcome.Stderr = stdout.Snapshot(), stderr.Snapshot()
	if state := command.ProcessState; state != nil {
		outcome.ExitCode = state.ExitCode()
		if outcome.ExitCode < 0 {
			outcome.Signal = state.String()
		}
	}
	var exitError *exec.ExitError
	if waitError != nil && !errors.As(waitError, &exitError) && outcome.ExitCode == 0 {
		outcome.ExitCode = -1
		outcome.StartError = waitError.Error()
	}
	if ctx.Err() != nil {
		outcome.Cancelled = true
	}
	return outcome
}

// #endregion 🏃️Runner
