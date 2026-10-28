#!/usr/bin/env python3
"""🪟 Edits of batch `pin` applied in place to a root: python3 -I edit.py <root>. On Windows the launcher resolves the pinned runtime from the user cache first and copies a runtime that only the tools partition holds into the user cache (once, without a download), so no `repo` command runs its `bun.exe` from the tools partition: a running image pins its directory and the partition is attached and detached by renaming it. Laws for the order, the adoption, the race, the refused adoption and the pin itself (Windows host) come with it."""
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
full(L + "/🧪️tests/🪟️runtime-outside-partition/🐹️.go", "pin.go")
full(L + "/🧪️tests/🪟️runtime-outside-partition/🥒️.feature", "pin.feature")
ADOPT = '''// 🪟️outsideToolsPartition tells whether the runtime must never run from the tools partition: a running Windows image pins the directory it lives in, and the partition is attached and detached by renaming it.
func (q ProvisionRequest) outsideToolsPartition() bool { return q.Platform == "win32" }

// 🪟️adopt copies an installed runtime of the tools partition into the user cache (executable and receipt, staged beside the destination and renamed into place, so peers that adopt at the same moment end with one valid copy) and answers the copy; any failure leaves the caller on the tools-partition copy.
func (q ProvisionRequest) adopt(from Install) (Install, error) {
	row, err := q.row()
	if err != nil {
		return Install{}, err
	}
	_, final, err := q.StoreDirectories(row)
	if err != nil {
		return Install{}, err
	}
	name := q.Runtime.ExecutableName()
	if len(filepath.Join(final, name)) > q.Registry.Cache.MaximumPathLength {
		return Install{}, Failure{ID: "pathTooLong", Values: map[string]string{"path": final, "limit": strconv.Itoa(q.Registry.Cache.MaximumPathLength)}}
	}
	if err := os.MkdirAll(filepath.Dir(final), 0o755); err != nil {
		return Install{}, err
	}
	token := make([]byte, 8)
	_, _ = rand.Read(token)
	staging := filepath.Join(filepath.Dir(final), ".prepare-"+hex.EncodeToString(token))
	if err := os.Mkdir(staging, 0o755); err != nil {
		return Install{}, err
	}
	defer os.RemoveAll(staging)
	for _, file := range []string{name, receiptName} {
		if err := copyInstalled(filepath.Join(from.Directory, file), filepath.Join(staging, file)); err != nil {
			return Install{}, err
		}
	}
	if !verifyInstalled(staging, name, row) {
		return Install{}, Failure{ID: "integrityFailed", Values: map[string]string{"what": "copy", "archive": row.Archive, "actual": "unverified", "expected": row.Sha256}}
	}
	if !verifyInstalled(final, name, row) {
		_ = os.RemoveAll(final)
	}
	if err := os.Rename(staging, final); err != nil && !verifyInstalled(final, name, row) {
		return Install{}, err
	}
	return Install{Executable: filepath.Join(final, name), Directory: final, Origin: "user-cache"}, nil
}

// 📄️copyInstalled copies one file of an install and keeps its permission bits.
func copyInstalled(source string, destination string) error {
	in, err := os.Open(source)
	if err != nil {
		return err
	}
	defer in.Close()
	info, err := in.Stat()
	if err != nil {
		return err
	}
	out, err := os.OpenFile(destination, os.O_CREATE|os.O_EXCL|os.O_WRONLY, info.Mode().Perm())
	if err != nil {
		return err
	}
	if _, err := io.Copy(out, in); err != nil {
		_ = out.Close()
		return err
	}
	return out.Close()
}

func (q ProvisionRequest) lock('''
edit(G + "/📥️runtime-provision/🐹️.go", [
 ("// 🔎️Locate answers an installed runtime without side effects: the tools partition first, then the user cache.",
  "// 🔎️Locate answers an installed runtime without side effects: the tools partition first, then the user cache (on Windows the other way round, see outsideToolsPartition)."),
 ('\tfor _, candidate := range []struct{ directory, origin string }{{tools, "tools-partition"}, {user, "user-cache"}} {\n',
  '\tcandidates := []struct{ directory, origin string }{{tools, "tools-partition"}, {user, "user-cache"}}\n\tif q.outsideToolsPartition() {\n\t\tcandidates[0], candidates[1] = candidates[1], candidates[0]\n\t}\n\tfor _, candidate := range candidates {\n'),
 ('func (q ProvisionRequest) Ensure(ctx context.Context) (Install, error) {\n\tif found, ok, err := q.Locate(); ok || err != nil {\n\t\treturn found, err\n\t}\n',
  'func (q ProvisionRequest) Ensure(ctx context.Context) (Install, error) {\n\tif found, ok, err := q.Locate(); ok || err != nil {\n\t\tif ok && err == nil && found.Origin == "tools-partition" && q.outsideToolsPartition() {\n\t\t\tif adopted, failure := q.adopt(found); failure == nil {\n\t\t\t\treturn adopted, nil\n\t\t\t}\n\t\t}\n\t\treturn found, err\n\t}\n'),
 ("func (q ProvisionRequest) lock(", ADOPT),
])
print("pin edits applied to", ROOT)
