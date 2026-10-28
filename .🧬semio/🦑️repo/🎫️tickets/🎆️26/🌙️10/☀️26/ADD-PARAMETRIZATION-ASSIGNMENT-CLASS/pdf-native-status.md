> Superseded by the later successful native compile: the parent located an existing pinned Tectonic executable, a local bundle exposed through its file URL, and prepared fonts matching the original source hashes. The actual assignment class compiled from the original product sources without copying them into the ticket. The observations below record the earlier registry/admission state; they no longer describe PDF creation availability. See `latex-product.md` and the parent compiler evidence.

# Native PDF Status

The current registered native route cannot create the requested PDF without repairing unrelated repository infrastructure.

Read-only checks on the reopened ticket:

- `repo tools list --json` reports Tectonic 0.16.9 as `source: missing`, `command: null`. Its Windows slot is `.cache/b/t/m/tectonic/131a24604785a9600989a3d91225f597df52ac06f00aeffe86fd529f99ee5cdd/tectonic`; the current native tool directory has only `native`.
- `.cache/b/l/fixed.dir` is still absent.
- `repo tools verify tectonic` enters native source admission and fails with the same `ENOENT .cache/b/l/fixed.dir` before executing tool verification.
- The print compilation interface consumes retained, explicitly prepared Tectonic, TeX and font publications. A compiler path by itself does not satisfy this interface.

No source or cache changes, tool installation, compiler invocation or direct compiler bypass was performed. The parent is producing the explicitly requested standalone deliverable with the bundled PDF authoring tools; that is separate from a successful native print-class build.
