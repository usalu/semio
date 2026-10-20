// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Child process tests: bounded capture, typed progress lines, stdin isolation, interruption and kill after the grace period. The children are POSIX shell scripts, so the cases run where a POSIX shell exists.

// #endregion 🧲️Header

package main

import (
	"context"
	"os"
	"os/exec"
	"strings"
	"sync"
	"testing"
	"time"
)

// #region 🏃️BuildProcessTests

func shellSpec(t *testing.T, script string, arguments ...string) ProcessSpec {
	t.Helper()
	if _, err := exec.LookPath("sh"); err != nil {
		t.Skip("no POSIX shell")
	}
	return ProcessSpec{Binary: "sh", Prefix: []string{"-c", script, "sh"}, Argv: arguments, Dir: t.TempDir(), Env: os.Environ(), Grace: 5 * time.Second, HeadMax: 1 << 20, StdoutMax: 4096, StderrMax: 4096}
}

type eventLog struct {
	mu     sync.Mutex
	events []ProcessEvent
}

func (log *eventLog) emit(event ProcessEvent) {
	log.mu.Lock()
	defer log.mu.Unlock()
	log.events = append(log.events, event)
}

func (log *eventLog) snapshot() []ProcessEvent {
	log.mu.Lock()
	defer log.mu.Unlock()
	return append([]ProcessEvent(nil), log.events...)
}

func TestProcessCapturesOutputAndExitCode(t *testing.T) {
	spec := shellSpec(t, `printf '{"a":1}'; echo "problem" >&2; exit 3`)
	outcome := ExecRunner{}.Run(context.Background(), spec, func(ProcessEvent) {})
	if !outcome.Started || outcome.ExitCode != 3 || outcome.Cancelled {
		t.Fatalf("outcome %+v", outcome)
	}
	if string(outcome.Stdout.Head) != `{"a":1}` || strings.TrimSpace(string(outcome.Stderr.Tail)) != "problem" {
		t.Fatalf("captured %q / %q", outcome.Stdout.Head, outcome.Stderr.Tail)
	}
}

func TestProcessTurnsTypedProgressLinesIntoEvents(t *testing.T) {
	script := `printf '%s\n' '[script] {"owner":"repository:build","command":"build:discover","stage":"running"}' >&2
printf '%s\n' '[script] {"owner":"repository:build","command":"repo","stage":"running","activity":{"resource":"lease-queue:x","mode":"shared","elapsedMs":0}}' >&2
printf '%s\n' '[script] repo · Waiting for access to x (shared): position 2 of 3' >&2
echo "plain stderr" >&2`
	log := &eventLog{}
	outcome := ExecRunner{}.Run(context.Background(), shellSpec(t, script), log.emit)
	events := log.snapshot()
	if outcome.ProgressEvents != 2 || len(events) != 2 {
		t.Fatalf("events %+v outcome %d", events, outcome.ProgressEvents)
	}
	if events[0].Script == nil || events[0].Message != "build:discover running" {
		t.Fatalf("a record without a human line is described by the server: %+v", events[0])
	}
	if events[1].Script == nil || events[1].Script.Activity == nil || events[1].Script.Activity.Mode != "shared" || !strings.Contains(events[1].Message, "position 2 of 3") {
		t.Fatalf("the human line explains its record: %+v", events[1])
	}
	if strings.Contains(string(outcome.Stderr.Tail), "[script]") || !strings.Contains(string(outcome.Stderr.Tail), "plain stderr") {
		t.Fatalf("progress lines must not pollute stderr: %q", outcome.Stderr.Tail)
	}
}

func TestProcessReadsNoStandardInput(t *testing.T) {
	outcome := ExecRunner{}.Run(context.Background(), shellSpec(t, `cat | wc -c | tr -d ' '`), func(ProcessEvent) {})
	if strings.TrimSpace(string(outcome.Stdout.Head)) != "0" {
		t.Fatalf("the child must not read the protocol stream: %q", outcome.Stdout.Head)
	}
}

func TestProcessBoundsCapturedOutput(t *testing.T) {
	spec := shellSpec(t, `yes 0123456789 | head -c 5000000`)
	spec.HeadMax = 1 << 16
	outcome := ExecRunner{}.Run(context.Background(), spec, func(ProcessEvent) {})
	stdout := outcome.Stdout
	if stdout.Total != 5000000 || len(stdout.Head) > 1<<16 || len(stdout.Tail) > spec.StdoutMax || !stdout.Overflow || !stdout.Truncated {
		t.Fatalf("capture is not bounded: total=%d head=%d tail=%d overflow=%v", stdout.Total, len(stdout.Head), len(stdout.Tail), stdout.Overflow)
	}
}

func TestProcessInterruptsOnCancelAndAllowsCleanup(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	spec := shellSpec(t, `trap 'echo cleaned >&2; exit 130' INT; echo started; while :; do sleep 0.05; done`)
	started := time.Now()
	go func() {
		time.Sleep(300 * time.Millisecond)
		cancel()
	}()
	outcome := ExecRunner{}.Run(ctx, spec, func(ProcessEvent) {})
	if !outcome.Cancelled || outcome.ExitCode != 130 || !strings.Contains(string(outcome.Stderr.Tail), "cleaned") {
		t.Fatalf("the child must be interrupted and clean up: %+v stderr=%q", outcome, outcome.Stderr.Tail)
	}
	if time.Since(started) > 4*time.Second {
		t.Fatal("interruption took the grace period")
	}
}

func TestProcessIsKilledAfterTheGracePeriod(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	spec := shellSpec(t, `trap '' INT; echo started; exec sleep 30`)
	spec.Grace = 300 * time.Millisecond
	go func() {
		time.Sleep(200 * time.Millisecond)
		cancel()
	}()
	started := time.Now()
	outcome := ExecRunner{}.Run(ctx, spec, func(ProcessEvent) {})
	if !outcome.Cancelled || outcome.ExitCode == 0 || time.Since(started) > 10*time.Second {
		t.Fatalf("a child that ignores the interrupt must be killed: %+v after %s", outcome, time.Since(started))
	}
}

func TestProcessReportsStartFailureAndHeartbeat(t *testing.T) {
	outcome := ExecRunner{}.Run(context.Background(), ProcessSpec{Binary: "definitely-not-an-executable-repo", Dir: t.TempDir(), Env: os.Environ(), Grace: time.Second, HeadMax: 1024, StdoutMax: 1024, StderrMax: 1024}, func(ProcessEvent) {})
	if outcome.Started || outcome.StartError == "" || outcome.ExitCode != -1 {
		t.Fatalf("start failure %+v", outcome)
	}
	spec := shellSpec(t, `echo working; sleep 0.6`)
	spec.Heartbeat = 100 * time.Millisecond
	log := &eventLog{}
	ExecRunner{}.Run(context.Background(), spec, log.emit)
	beats := 0
	for _, event := range log.snapshot() {
		if event.Kind == "heartbeat" && event.Message == "working" {
			beats++
		}
	}
	if beats == 0 {
		t.Fatalf("a silent child must still show liveness: %+v", log.snapshot())
	}
}

func TestPrintableDropsTerminalEscapes(t *testing.T) {
	got := Printable([]byte("\x1b[31mred\x1b[0m \x1b]0;title\x07ok \xff"))
	if got != "red ok �" {
		t.Fatalf("got %q", got)
	}
}

// #endregion 🏃️BuildProcessTests
