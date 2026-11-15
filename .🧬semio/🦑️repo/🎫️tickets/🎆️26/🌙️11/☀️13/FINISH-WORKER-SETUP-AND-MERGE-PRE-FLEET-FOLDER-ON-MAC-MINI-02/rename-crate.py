"""✂️ Renames the crate semio-framework-replication-retained-field-preparation to …-retained-field-prep in every manifest, lock and source of the worktree, so its compiler output paths fit the 256 character path envelope below the canonical worktree root."""
import os, sys
root, apply = sys.argv[1], "--apply" in sys.argv
pairs = [(b"semio-framework-replication-retained-field-preparation", b"semio-framework-replication-retained-field-prep"), (b"semio_framework_replication_retained_field_preparation", b"semio_framework_replication_retained_field_prep")]
skip = {"node_modules", ".git", ".cache", "target", ".\U0001f9ecsemio"}
changed = []
for directory, names, files in os.walk(root):
    names[:] = [name for name in names if name not in skip]
    for name in files:
        path = os.path.join(directory, name)
        if os.path.islink(path) or os.path.getsize(path) > 64 << 20:
            continue
        with open(path, "rb") as handle:
            data = handle.read()
        if not any(old in data for old, _ in pairs):
            continue
        for old, new in pairs:
            data = data.replace(old, new)
        changed.append(path)
        if apply:
            with open(path, "wb") as handle:
                handle.write(data)
print(len(changed), "files")
for path in changed:
    print(os.path.relpath(path, root))
