#!/usr/bin/env python3
"""🪟 Edits of batch `winlaunch` applied in place to a root (a scratch copy of the live files or a mirror): python3 -I edit.py <root>. Anchored replacements (each anchor must occur exactly once) for the shared Go files and the service session module, full sources from `src/` for the two Windows members and the new Windows-only test case."""
import os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = sys.argv[1]
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher"
G = L + "/📦️packages/🐹️go"
def edit(rel, pairs):
    path = os.path.join(ROOT, rel); text = open(path, encoding="utf-8").read()
    for old, new in pairs:
        if new in text: continue
        if text.count(old) != 1: sys.exit("anchor %d× in %s: %s" % (text.count(old), rel, old[:70]))
        text = text.replace(old, new)
    open(path, "w", encoding="utf-8").write(text)
def full(rel, source):
    os.makedirs(os.path.dirname(os.path.join(ROOT, rel)), exist_ok=True)
    shutil.copyfile(os.path.join(HERE, "src", source), os.path.join(ROOT, rel))
full(G + "/🪟️windows-host/🐹️.go", "windows-host.go")
full(G + "/🛰️resident-windows/🐹️.go", "resident-windows.go")
full(L + "/🧪️tests/🪟️windows-contract/🐹️.go", "windows-contract.go")
full(L + "/🧪️tests/🪟️windows-contract/🥒️.feature", "windows-contract.feature")
edit(G + "/🐚️unix-host/🐹️.go", [
 ("// 🐚️unix-host hands the process over to the runtime in place, so the PID, streams, signals and exit code are the runtime's own, and reads the system language list.",
  "// 🐚️unix-host hands the process over to the runtime in place, so the PID, streams, signals and exit code are the runtime's own, names a worktree by its resolved links, and reads the system language list."),
 ('\t"os/signal"\n\t"regexp"\n', '\t"os/signal"\n\t"path/filepath"\n\t"regexp"\n'),
 ("// 🖥️systemLanguages lists the macOS preferred languages",
  "// 🪢️containChild needs no job on POSIX: the runtime that `exec` replaced this process with already is the process the caller ends.\nfunc containChild(int) func() { return func() {} }\n\n// 🧭️canonicalRoot resolves every link of the root, as the service's `realpathSync` does.\nfunc canonicalRoot(path string) string {\n\tif real, err := filepath.EvalSymlinks(path); err == nil {\n\t\treturn real\n\t}\n\treturn path\n}\n\n// 🔁️renameTransient is false on POSIX: a rename there is never refused for a moment because someone holds a file open.\nfunc renameTransient(error) bool { return false }\n\n// 🖥️systemLanguages lists the macOS preferred languages"),
])
edit(G + "/🏗️entry/🐹️.go", [
 ("\tif err := command.Start(); err != nil {\n\t\treturn err\n\t}\n\tdone := make(chan struct{})",
  "\tif err := command.Start(); err != nil {\n\t\treturn err\n\t}\n\trelease := containChild(command.Process.Pid)\n\tdefer release()\n\tdone := make(chan struct{})"),
 ("Windows hands over this way, and a caller that continues after the runtime finished (the help of the full command) selects it as its HandOff.",
  "Windows hands over this way (the runtime ends with the launcher through a job object), and a caller that continues after the runtime finished (the help of the full command) selects it as its HandOff."),
])
edit(G + "/🛰️resident/🐹️.go", [
 ('\t"crypto/hmac"\n', '\t"crypto/hmac"\n\t"crypto/rand"\n'),
 ("\treal, err := filepath.EvalSymlinks(root)\n\tif err != nil {\n\t\treal = root\n\t}\n", "\treal := canonicalRoot(root)\n"),
 ("\tidentifier := make([]byte, 8)\n\tif file, openErr := os.Open(\"/dev/urandom\"); openErr == nil {\n\t\t_, _ = io.ReadFull(file, identifier)\n\t\tfile.Close()\n\t} else {\n\t\tsum := sha256.Sum256([]byte(strconv.FormatInt(time.Now().UnixNano(), 10)))\n\t\tcopy(identifier, sum[:8])\n\t}\n",
  "\tidentifier := make([]byte, 8)\n\t_, _ = rand.Read(identifier)\n"),
])
edit(G + "/🧭️workspace-root/🐹️.go", [
 ("// 🧱️IsRootNotFound reports whether err is a missing root.",
  "// 🪟️FinalPathSpelling turns a Windows final path into the name a shell uses, exactly as libuv's `realpath` does: `\\\\?\\UNC\\server\\share` becomes `\\\\server\\share`, `\\\\?\\C:\\x` becomes `C:\\x`, anything else stays. See https://learn.microsoft.com/windows/win32/fileio/naming-a-file#win32-file-namespaces\nfunc FinalPathSpelling(final string) string {\n\tif rest, found := strings.CutPrefix(final, `\\\\?\\UNC\\`); found {\n\t\treturn `\\\\` + rest\n\t}\n\treturn strings.TrimPrefix(final, `\\\\?\\`)\n}\n\n// 🧱️IsRootNotFound reports whether err is a missing root."),
])
edit(G + "/📥️runtime-provision/🐹️.go", [
 ("\tif err := os.Rename(staging, final); err != nil {",
  "\tif err := renameRetrying(os.Rename, renameTransient, time.Sleep, staging, final); err != nil {"),
 ("func (q ProvisionRequest) download(",
  "// 🔁️renameRetrying publishes by rename and repeats a refusal the host calls transient (Windows: a scanner or indexer that holds the fresh executable for a moment) after 10, 20, 40 … 1280 ms, 2.55 s in all, like the Go command's own robust rename; any other error, and the last transient one, is returned. See https://cs.opensource.google/go/go/+/refs/tags/go1.26.0:src/cmd/go/internal/robustio/robustio_windows.go\nfunc renameRetrying(rename func(string, string) error, transient func(error) bool, sleep func(time.Duration), from, to string) error {\n\tfor wait := 10 * time.Millisecond; ; wait *= 2 {\n\t\terr := rename(from, to)\n\t\tif err == nil || !transient(err) || wait > 1280*time.Millisecond {\n\t\t\treturn err\n\t\t}\n\t\tsleep(wait)\n\t}\n}\n\nfunc (q ProvisionRequest) download("),
])
edit(L + "/🧪️tests/🧭️root-contract/🐹️.go", [
 ("// 🧭️Every corpus case resolves to the expected root, or to none.",
  "// 🪟️A Windows final path reads as the shell spelling libuv's `realpath` gives it (the service hashes that text): the Win32 file namespace prefix goes, a UNC volume keeps its two leading separators, any other text stays.\nfunc TestFinalPathSpelling(t *testing.T) {\n\tfor final, want := range map[string]string{\n\t\t`\\\\?\\C:\\Users\\ada\\semio`:     `C:\\Users\\ada\\semio`,\n\t\t`\\\\?\\c:\\semio`:               `c:\\semio`,\n\t\t`\\\\?\\UNC\\server\\share\\semio`: `\\\\server\\share\\semio`,\n\t\t`\\\\server\\share\\semio`:       `\\\\server\\share\\semio`,\n\t\t`C:\\semio`:                   `C:\\semio`,\n\t\t`\\\\.\\C:\\semio`:               `\\\\.\\C:\\semio`,\n\t} {\n\t\tif got := FinalPathSpelling(final); got != want {\n\t\t\tt.Errorf(\"%s: %s, want %s\", final, got, want)\n\t\t}\n\t}\n}\n\n// 🧭️Every corpus case resolves to the expected root, or to none."),
])
edit(L + "/🧪️tests/🧭️root-contract/🥒️.feature", [
 ("    And a malformed package.json is refused instead of skipped\n",
  "    And a malformed package.json is refused instead of skipped\n  Scenario: A Windows final path reads as the shell spelling\n    Given a final path of the kernel with or without the Win32 file namespace prefix\n    When the launcher names the worktree\n    Then it strips the prefix exactly as the service's runtime does\n"),
])
edit(L + "/🧪️tests/📥️provision-contract/🐹️.go", [
 ("func TestProvisioningRefusals(t *testing.T) {",
  "// 🔁️A transient rename refusal is repeated after 10, 20, 40 … 1280 ms (eight waits, 2.55 s in all); success ends the waiting, any other error ends it at once, and the last transient error is reported.\nfunc TestProvisioningRenameRetriesTransient(t *testing.T) {\n\tbusy, other := errors.New(\"busy\"), errors.New(\"other\")\n\trun := func(outcomes ...error) (error, []time.Duration, int) {\n\t\tvar waits []time.Duration\n\t\tcalls := 0\n\t\trename := func(string, string) error {\n\t\t\tcalls++\n\t\t\tif calls <= len(outcomes) {\n\t\t\t\treturn outcomes[calls-1]\n\t\t\t}\n\t\t\treturn busy\n\t\t}\n\t\terr := renameRetrying(rename, func(err error) bool { return errors.Is(err, busy) }, func(wait time.Duration) { waits = append(waits, wait) }, \"staging\", \"final\")\n\t\treturn err, waits, calls\n\t}\n\tms := time.Millisecond\n\tif err, waits, calls := run(busy, busy, nil); err != nil || calls != 3 || len(waits) != 2 || waits[0] != 10*ms || waits[1] != 20*ms {\n\t\tt.Errorf(\"retry then land: %v %v %d\", err, waits, calls)\n\t}\n\tif err, waits, calls := run(other); !errors.Is(err, other) || calls != 1 || len(waits) != 0 {\n\t\tt.Errorf(\"other error: %v %v %d\", err, waits, calls)\n\t}\n\terr, waits, calls := run()\n\tvar total time.Duration\n\tfor _, wait := range waits {\n\t\ttotal += wait\n\t}\n\tif !errors.Is(err, busy) || calls != 9 || len(waits) != 8 || waits[7] != 1280*ms || total != 2550*ms {\n\t\tt.Errorf(\"exhausted: %v %v %d\", err, waits, calls)\n\t}\n\tif renameTransient(os.ErrNotExist) || renameTransient(nil) {\n\t\tt.Error(\"a missing file or no error is never transient\")\n\t}\n}\n\nfunc TestProvisioningRefusals(t *testing.T) {"),
])
edit(L + "/🧪️tests/📥️provision-contract/🥒️.feature", [
 ("    And the tools partition is preferred over the user cache\n",
  "    And the tools partition is preferred over the user cache\n  Scenario: A rename a scanner refuses for a moment is repeated\n    Given a publication rename the host refuses as transient\n    When provisioning publishes the runtime\n    Then it repeats after 10 to 1280 ms, 2.55 s in all, and reports any other error at once\n"),
])
S = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🛰️service"
edit(S + "/🔑️session/🟨️.mjs", [
 ("/** 🏠 The realpath of a worktree root: the launcher and the service must hash the same spelling. */\nexport function canonicalRootV1(root){try{return realpathSync(root);}catch{return root;}}",
  "/** 🏠 The realpath of a worktree root: the launcher and the service must hash the same spelling. On Windows that is the kernel's final path (`realpathSync.native`, i.e. `GetFinalPathNameByHandleW`), the one name every drive-letter and component case of the checkout maps to and the one the Go launcher asks the kernel for; elsewhere the resolved links. See https://learn.microsoft.com/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlew */\nexport function canonicalRootV1(root,platform=process.platform){try{return platform===\"win32\"?realpathSync.native(root):realpathSync(root);}catch{return root;}}"),
])
edit(S + "/🧪️tests/🟦️.ts", [
 ("import {attenuateCapabilityV1,canonicalJsonV1,capabilityIdV1,", "import {attenuateCapabilityV1,canonicalJsonV1,canonicalRootV1,capabilityIdV1,"),
 ("test(\"without XDG_RUNTIME_DIR the endpoint directory follows TMPDIR only",
  "test(\"the worktree root is the kernel's final path on Windows (realpathSync.native, the GetFinalPathNameByHandleW name the Go launcher hashes) and the resolved links elsewhere; a missing root stays as given\",()=>{\n const native=spyOn(realpathSync,\"native\"),directory=mkdtempSync(join(tmpdir(),\"canon-\"));\n try{\n  const actual=canonicalRootV1(directory,\"win32\");expect(native.mock.calls).toEqual([[directory]]);expect(actual).toBe(realpathSync.native(directory));\n  const calls=native.mock.calls.length;expect(canonicalRootV1(directory,\"darwin\")).toBe(realpathSync(directory));expect(native.mock.calls.length).toBe(calls);\n  for(const platform of[\"win32\",\"linux\"]as const)expect(canonicalRootV1(join(directory,\"missing\"),platform)).toBe(join(directory,\"missing\"));\n }finally{native.mockRestore();rmSync(directory,{recursive:true,force:true});}\n});\n\ntest.skipIf(process.platform!==\"win32\")(\"on Windows every drive-letter and component case of one checkout gives one worktree id\",()=>{\n const directory=mkdtempSync(join(tmpdir(),\"Canon-Case-\")),variant=directory.slice(0,1).toLowerCase()+directory.slice(1).toUpperCase();\n try{expect(worktreeIdV1(canonicalRootV1(variant))).toBe(worktreeIdV1(canonicalRootV1(directory)));}finally{rmSync(directory,{recursive:true,force:true});}\n});\n\ntest(\"without XDG_RUNTIME_DIR the endpoint directory follows TMPDIR only"),
])
print("winlaunch edits applied to", ROOT)
