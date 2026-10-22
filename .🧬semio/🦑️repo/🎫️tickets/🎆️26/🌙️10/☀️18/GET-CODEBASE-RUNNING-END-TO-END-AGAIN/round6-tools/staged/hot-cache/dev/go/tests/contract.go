// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🛰️resident-contract proves the thin client of the resident service: the same endpoint, capability and eligibility vectors as the JavaScript implementation (shared language-neutral fixtures), the streamed answer, and silent fall back whenever no service can answer.

// #endregion 🧲️Header

package launcher

import (
	"bufio"
	"bytes"
	"context"
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"io"
	"net"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"sync"
	"testing"
	"time"
)

const residentVectors = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🧫️fixtures/🔣️.json"
const residentEligibility = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🧫️fixtures/📋️eligibility.json"

type residentVectorFile struct {
	Capabilities []struct {
		Key     string `json:"key"`
		ID      string `json:"id"`
		Caveats []any  `json:"caveats"`
		Token   string `json:"token"`
	} `json:"capabilities"`
	Endpoints []struct {
		Root string `json:"root"`
		ID   string `json:"id"`
	} `json:"endpoints"`
}

// #region 🛰️Vectors

// 🛰️The worktree id and the capability token equal the values computed independently by Python (hashlib, hmac) for the JavaScript service.
func TestResidentVectorsMatchPython(t *testing.T) {
	if runtime.GOOS == "windows" {
		t.Skip("posix paths in the fixture")
	}
	var vectors residentVectorFile
	readFixture(t, residentVectors, &vectors)
	if len(vectors.Endpoints) == 0 || len(vectors.Capabilities) == 0 {
		t.Fatal("fixture rows required")
	}
	for _, row := range vectors.Endpoints {
		if got := ResidentWorktreeID(row.Root); got != row.ID {
			t.Errorf("id of %q: %s, want %s", row.Root, got, row.ID)
		}
	}
	for _, row := range vectors.Capabilities {
		key, _ := hex.DecodeString(row.Key)
		if got := ResidentMint(key, row.ID, row.Caveats); got != row.Token {
			t.Errorf("token of %v:\n got %s\nwant %s", row.Caveats, got, row.Token)
		}
	}
}

// 🛰️Socket paths stay below the sockaddr limit, are per user and worktree, and an empty XDG directory counts as unset.
func TestResidentEndpointShapes(t *testing.T) {
	env := environment(map[string]string{"XDG_RUNTIME_DIR": "/run/user/1000"})
	a := ResidentEndpointsFor("/w/a", env, "linux", 1000, "/tmp")
	if a.Endpoint != "/run/user/1000/semio-repo-1000/"+ResidentWorktreeID("/w/a")+"/s" {
		t.Errorf("endpoint %s", a.Endpoint)
	}
	long := ResidentEndpointsFor("/w/a", environment(map[string]string{"XDG_RUNTIME_DIR": strings.Repeat("/x", 60)}), "linux", 1000, "/tmp")
	if len(long.Endpoint) >= 104 || !strings.HasPrefix(long.Endpoint, "/tmp/semio-repo-1000/") {
		t.Errorf("long endpoint %s", long.Endpoint)
	}
	if got := ResidentEndpointsFor("/w/a", environment(map[string]string{"XDG_RUNTIME_DIR": ""}), "linux", 7, "/var/t/").Directory; !strings.HasPrefix(got, "/var/t/semio-repo-7/") {
		t.Errorf("empty XDG: %s", got)
	}
	win := ResidentEndpointsFor(`C:\w`, environment(map[string]string{}), "windows", -1, `C:\t`)
	if win.Endpoint != `\\.\pipe\semio-repo--1-`+ResidentWorktreeID(`C:\w`) {
		t.Errorf("pipe %s", win.Endpoint)
	}
}

// 🛰️The eligibility and mode rules equal the JavaScript rules row by row.
func TestResidentEligibilityFixture(t *testing.T) {
	var fixture struct {
		Eligibility []struct {
			Args     []string `json:"args"`
			Eligible bool     `json:"eligible"`
		} `json:"eligibility"`
		Modes []struct {
			Args        []string          `json:"args"`
			Environment map[string]string `json:"environment"`
			Mode        string            `json:"mode"`
		} `json:"modes"`
		Agents []struct {
			Value    string `json:"value"`
			Fallback string `json:"fallback"`
			Agent    string `json:"agent"`
		} `json:"agents"`
	}
	readFixture(t, residentEligibility, &fixture)
	for _, row := range fixture.Agents {
		if got := ResidentAgentID(row.Value, row.Fallback); got != row.Agent {
			t.Errorf("agent(%q, %q) = %q, want %q", row.Value, row.Fallback, got, row.Agent)
		}
	}
	for _, row := range fixture.Eligibility {
		if got := ResidentEligible(row.Args); got != row.Eligible {
			t.Errorf("eligible(%v) = %v", row.Args, got)
		}
	}
	for _, row := range fixture.Modes {
		want := row.Mode
		if want == "invalid" {
			want = "off"
		}
		if got := ResidentMode(row.Args, environment(row.Environment)); got != want {
			t.Errorf("mode(%v) = %s, want %s", row.Args, got, want)
		}
	}
}

// #endregion 🛰️Vectors

// #region 🛰️Service

type residentFake struct {
	root     string
	env      map[string]string
	endpoint string
	frames   chan map[string]any
	reply    func(request map[string]any, write func(frame map[string]any))
	stdout   *bytes.Buffer
	stderr   *bytes.Buffer
}

// 🧷shortTempBase is the platform's temporary directory, or the POSIX root /tmp where that one is too long for a unix socket beneath it.
func shortTempBase() string {
	if runtime.GOOS == "windows" || len(os.TempDir()) <= 40 {
		return os.TempDir()
	}
	return "/tmp"
}

func residentSetup(t *testing.T, serve bool) *residentFake {
	t.Helper()
	if runtime.GOOS == "windows" {
		t.Skip("unix socket service")
	}
	real, _ := filepath.EvalSymlinks(testRoot(t))
	base, err := os.MkdirTemp(shortTempBase(), "sr-")
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { os.RemoveAll(base) })
	fake := &residentFake{root: real, env: map[string]string{"XDG_RUNTIME_DIR": base, "SEMIO_LANGUAGE": "de"}, frames: make(chan map[string]any, 16), stdout: &bytes.Buffer{}, stderr: &bytes.Buffer{}}
	paths := ResidentEndpointsFor(real, environment(fake.env), residentGOOS, os.Getuid(), os.TempDir())
	if err := os.MkdirAll(paths.Directory, 0o700); err != nil {
		t.Fatal(err)
	}
	_ = os.Chmod(paths.Directory, 0o700)
	_ = os.WriteFile(paths.Key, []byte("0123456789abcdef0123456789abcdef"), 0o600)
	fake.endpoint = paths.Endpoint
	oldOut, oldErr := residentStdout, residentStderr
	residentStdout, residentStderr = fake.stdout, fake.stderr
	t.Cleanup(func() { residentStdout, residentStderr = oldOut, oldErr })
	if serve {
		listener, err := net.Listen("unix", paths.Endpoint)
		if err != nil {
			t.Fatal(err)
		}
		t.Cleanup(func() { listener.Close() })
		go func() {
			for {
				connection, err := listener.Accept()
				if err != nil {
					return
				}
				go fake.handle(connection)
			}
		}()
	}
	return fake
}

func (f *residentFake) handle(connection net.Conn) {
	defer connection.Close()
	reader := bufio.NewReader(connection)
	var mu sync.Mutex
	write := func(frame map[string]any) {
		mu.Lock()
		defer mu.Unlock()
		if frame["t"] == "close" {
			_ = connection.Close()
			return
		}
		raw, _ := json.Marshal(frame)
		_, _ = connection.Write(append(raw, '\n'))
	}
	for {
		line, err := reader.ReadBytes('\n')
		if err != nil {
			return
		}
		var frame map[string]any
		if json.Unmarshal(line, &frame) != nil {
			return
		}
		f.frames <- frame
		if frame["t"] == "request" && f.reply != nil {
			f.reply(frame, write)
		}
	}
}

func (f *residentFake) host(t *testing.T) Host {
	t.Helper()
	host, _ := recordingHost(t, f.env)
	host.Stderr = func(text string) { f.stderr.WriteString(text + "\n") }
	return host
}

func residentTimeout(t *testing.T, channel chan map[string]any) map[string]any {
	t.Helper()
	select {
	case frame := <-channel:
		return frame
	case <-time.After(3 * time.Second):
		t.Fatal("no frame")
		return nil
	}
}

// 🛰️A served request sends one hello and one request in order with the preferred language first, the bound environment and the launcher capability, and streams output byte for byte before answering with the exit code of the service.
func TestResidentForwardServesAndStreams(t *testing.T) {
	fake := residentSetup(t, true)
	fake.env["SEMIO_BUILD_TOOL"] = "bazel"
	fake.reply = func(request map[string]any, write func(map[string]any)) {
		write(map[string]any{"t": "accepted", "id": 1, "queue": 0})
		write(map[string]any{"t": "output", "id": 1, "stream": "stdout", "text": "ran a:build\n"})
		write(map[string]any{"t": "output", "id": 1, "stream": "stderr", "text": "warn äö😀\n"})
		write(map[string]any{"t": "result", "id": 1, "exit": 3, "served": "executed", "timing": map[string]any{}})
	}
	handled, code := fake.host(t).ForwardResident(context.Background(), testRoot(t), "build", []string{"run", "a:build", "--output=stream"})
	if !handled || code != 3 {
		t.Fatalf("handled=%v code=%d", handled, code)
	}
	hello, request := residentTimeout(t, fake.frames), residentTimeout(t, fake.frames)
	if hello["t"] != "hello" || request["t"] != "request" {
		t.Fatalf("frames %v %v", hello, request)
	}
	languages := hello["languages"].([]any)
	if languages[0] != "de" || languages[1] != "en" {
		t.Errorf("languages %v", languages)
	}
	if request["command"] != "run" || request["args"].([]any)[0] != "a:build" || len(request["args"].([]any)) != 1 {
		t.Errorf("request %v", request)
	}
	if request["env"].(map[string]any)["SEMIO_BUILD_TOOL"] != "bazel" {
		t.Errorf("bound environment %v", request["env"])
	}
	if hello["cap"] == "" || !strings.HasPrefix(hello["cap"].(string), "v1.") {
		t.Errorf("capability %v", hello["cap"])
	}
	if lane := hello["client"].(map[string]any)["lane"]; lane != "launcher" {
		t.Errorf("lane %v", lane)
	}
	caveats := residentCaveats(t, hello["cap"].(string))
	if len(caveats) != 4 || caveats[2]["k"] != "worktree" || caveats[2]["v"] != ResidentWorktreeID(fake.root) || caveats[3]["k"] != "agent" || caveats[3]["v"] != "launcher" {
		t.Errorf("caveats %v", caveats)
	}
	if fake.stdout.String() != "ran a:build\n" || fake.stderr.String() != "warn äö😀\n" {
		t.Errorf("streams %q %q", fake.stdout.String(), fake.stderr.String())
	}
}

func residentCaveats(t *testing.T, token string) []map[string]any {
	t.Helper()
	parts := strings.Split(token, ".")
	if len(parts) != 3 {
		t.Fatalf("token %q", token)
	}
	raw, err := base64.RawURLEncoding.DecodeString(parts[1])
	if err != nil {
		t.Fatal(err)
	}
	var payload struct {
		Caveats []map[string]any `json:"caveats"`
	}
	if err := json.Unmarshal(raw, &payload); err != nil {
		t.Fatal(err)
	}
	return payload.Caveats
}

// 🛰️An agent identity from SEMIO_AGENT_ID becomes the lane and the agent caveat, and a token the service issued for an agent session (SEMIO_SERVICE_TOKEN) is sent as it is, never replaced by a self-minted one.
func TestResidentAgentIdentityAndIssuedToken(t *testing.T) {
	fake := residentSetup(t, true)
	fake.env["SEMIO_AGENT_ID"] = "codex-1"
	fake.reply = func(request map[string]any, write func(map[string]any)) {
		write(map[string]any{"t": "result", "id": 1, "exit": 0, "served": "memo", "timing": map[string]any{}})
	}
	if handled, _ := fake.host(t).ForwardResident(context.Background(), testRoot(t), "build", []string{"run", "a:build"}); !handled {
		t.Fatal("not handled")
	}
	hello, _ := residentTimeout(t, fake.frames), residentTimeout(t, fake.frames)
	if hello["client"].(map[string]any)["lane"] != "codex-1" || residentCaveats(t, hello["cap"].(string))[3]["v"] != "codex-1" {
		t.Errorf("hello %v", hello)
	}
	fake.env["SEMIO_SERVICE_TOKEN"] = "v1.issued.token"
	if handled, _ := fake.host(t).ForwardResident(context.Background(), testRoot(t), "build", []string{"run", "a:build"}); !handled {
		t.Fatal("not handled")
	}
	hello, _ = residentTimeout(t, fake.frames), residentTimeout(t, fake.frames)
	if hello["cap"] != "v1.issued.token" {
		t.Errorf("issued token replaced: %v", hello["cap"])
	}
}

// 🛰️Everything that is not a plain served request steps aside silently so that the normal path decides: no service, an untrusted runtime directory, a missing key, an ineligible command, mode off, a fallback refusal, a broken stream before any output, a language that cannot be resolved.
func TestResidentFallsBackSilently(t *testing.T) {
	root := testRoot(t)
	run := func(fake *residentFake, mode string, args ...string) (bool, int) {
		if mode != "" {
			fake.env["SEMIO_BUILD_SERVICE"] = mode
		}
		return fake.host(t).ForwardResident(context.Background(), root, "build", args)
	}
	silent := func(name string, fake *residentFake, handled bool, code int) {
		t.Helper()
		if handled || code != 0 || fake.stdout.Len() != 0 || fake.stderr.Len() != 0 {
			t.Errorf("%s: handled=%v code=%d out=%q err=%q", name, handled, code, fake.stdout.String(), fake.stderr.String())
		}
	}
	none := residentSetup(t, false)
	h, c := run(none, "", "run", "a:build")
	silent("no service", none, h, c)
	listening := residentSetup(t, true)
	h, c = run(listening, "off", "run", "a:build")
	silent("mode off", listening, h, c)
	h, c = run(listening, "auto", "run", "a:build", "--skip-cache")
	silent("ineligible", listening, h, c)
	listening.env["SEMIO_LANGUAGE"] = "fr"
	h, c = run(listening, "auto", "run", "a:build")
	silent("unresolvable language", listening, h, c)
	listening.env["SEMIO_LANGUAGE"] = "de"
	listening.reply = func(request map[string]any, write func(map[string]any)) {
		write(map[string]any{"t": "refused", "id": 1, "code": "environment", "message": map[string]any{"en": "x", "de": "y"}, "retry": "fallback"})
	}
	h, c = run(listening, "auto", "run", "a:build")
	silent("refused with fallback", listening, h, c)
	paths := ResidentEndpointsFor(listening.root, environment(listening.env), residentGOOS, os.Getuid(), os.TempDir())
	_ = os.Chmod(paths.Directory, 0o755)
	h, c = run(listening, "auto", "run", "a:build")
	silent("directory open to others", listening, h, c)
	_ = os.Chmod(paths.Directory, 0o700)
	_ = os.Remove(paths.Key)
	h, c = run(listening, "auto", "run", "a:build")
	silent("no key", listening, h, c)
}

// 🛰️require turns every fall back into a refusal in the chosen language; busy is exit 75 with the service's own text and never a silent fall back.
func TestResidentRequireAndBusy(t *testing.T) {
	root := testRoot(t)
	for language, want := range map[string]string{"en": "no resident service answered", "de": "kein residenter Dienst"} {
		fake := residentSetup(t, false)
		fake.env["SEMIO_LANGUAGE"] = language
		fake.env["SEMIO_BUILD_SERVICE"] = "require"
		handled, code := fake.host(t).ForwardResident(context.Background(), root, "build", []string{"run", "a:build"})
		if !handled || code != 1 || !strings.Contains(fake.stderr.String(), want) {
			t.Errorf("%s: handled=%v code=%d stderr=%q", language, handled, code, fake.stderr.String())
		}
	}
	unresolved := residentSetup(t, false)
	unresolved.env["SEMIO_LANGUAGE"] = "fr"
	unresolved.env["SEMIO_BUILD_SERVICE"] = "require"
	handled, code := unresolved.host(t).ForwardResident(context.Background(), root, "build", []string{"run", "a:build"})
	if !handled || code != 1 || !strings.Contains(unresolved.stderr.String(), "no resident service answered") || !strings.Contains(unresolved.stderr.String(), "kein residenter Dienst") {
		t.Errorf("unresolved language must be refused in every supported language: handled=%v code=%d stderr=%q", handled, code, unresolved.stderr.String())
	}
	fake := residentSetup(t, true)
	fake.reply = func(request map[string]any, write func(map[string]any)) {
		write(map[string]any{"t": "refused", "id": 1, "code": "busy", "message": map[string]any{"en": "busy now", "de": "gerade ausgelastet"}, "retry": "later"})
	}
	handled, code = fake.host(t).ForwardResident(context.Background(), root, "build", []string{"run", "a:build"})
	if !handled || code != 75 || !strings.Contains(fake.stderr.String(), "gerade ausgelastet") {
		t.Errorf("busy: handled=%v code=%d stderr=%q", handled, code, fake.stderr.String())
	}
}

// 🛰️A service that disappears after it accepted the request is reported as exit 70 and is never run again silently.
func TestResidentLostAfterAcceptance(t *testing.T) {
	fake := residentSetup(t, true)
	fake.reply = func(request map[string]any, write func(map[string]any)) {
		write(map[string]any{"t": "accepted", "id": 1, "queue": 0})
		write(map[string]any{"t": "output", "id": 1, "stream": "stdout", "text": "half"})
		write(map[string]any{"t": "close"})
	}
	handled, code := fake.host(t).ForwardResident(context.Background(), testRoot(t), "build", []string{"run", "a:build"})
	if !handled || code != 70 || fake.stdout.String() != "half" {
		t.Errorf("handled=%v code=%d out=%q", handled, code, fake.stdout.String())
	}
}

// 🛰️Only build routes are forwarded.
func TestResidentLeavesOtherRoutesAlone(t *testing.T) {
	fake := residentSetup(t, true)
	for _, kind := range []string{"workspace", "script", "mcp"} {
		if handled, _ := fake.host(t).ForwardResident(context.Background(), testRoot(t), kind, []string{"run", "a:build"}); handled {
			t.Errorf("%s was forwarded", kind)
		}
	}
}

// 🛰️Against a running JavaScript service (SEMIO_RESIDENT_SERVICE_ROOT names its worktree and XDG_RUNTIME_DIR its runtime base) the client gets a real answer.
func TestResidentAgainstTheRealService(t *testing.T) {
	root, base := os.Getenv("SEMIO_RESIDENT_SERVICE_ROOT"), os.Getenv("XDG_RUNTIME_DIR")
	if root == "" || base == "" || runtime.GOOS == "windows" {
		t.Skip("no running service")
	}
	host, _ := recordingHost(t, map[string]string{"XDG_RUNTIME_DIR": base, "SEMIO_LANGUAGE": "en"})
	var out bytes.Buffer
	oldOut := residentStdout
	residentStdout = io.Writer(&out)
	defer func() { residentStdout = oldOut }()
	target := os.Getenv("SEMIO_RESIDENT_SERVICE_TARGET")
	if target == "" {
		target = "p1499:build"
	}
	handled, code := host.ForwardResident(context.Background(), root, "build", []string{"run", target})
	if !handled || code != 0 || out.Len() == 0 {
		t.Errorf("handled=%v code=%d out=%q", handled, code, out.String())
	}
}

// #endregion 🛰️Service
