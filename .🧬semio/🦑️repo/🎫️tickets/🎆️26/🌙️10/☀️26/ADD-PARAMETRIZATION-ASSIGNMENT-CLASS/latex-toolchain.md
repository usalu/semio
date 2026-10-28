# Existing LaTeX Toolchain Discovery

## Verified Compiler

No download or global installation was necessary. The repository's old cached compiler is available at:

`C:/git/semio/.🧬semio/🦑️repo/⚡️cache/tectonic/0.16.9/tectonic.exe`

A second identical copy is at:

`C:/git/semio/.🧬semio/🦑️repo/⚡️cache/tools/tectonic/0.16.9/x86_64-pc-windows-msvc/tectonic.exe`

Both files have SHA256 `a0a9a5eaf1a940d9a615ad78d35225ca59420c7984576c6402fffb3e9fb05ceb`, size 49,630,208 bytes. The existing adjacent `.toolchain.json` records version 0.16.9, target x86_64-pc-windows-msvc, pinned archive SHA256 `131a24604785a9600989a3d91225f597df52ac06f00aeffe86fd529f99ee5cdd`, and the matching extracted executable digest. The current source toolchain metadata and registry pin name the same archive.

The registered invocation:

```powershell
$env:SEMIO_TOOL_TECTONIC = 'C:/git/semio/.🧬semio/🦑️repo/⚡️cache/tectonic/0.16.9/tectonic.exe'
repo tools exec --light tectonic -- --version
```

returned `Tectonic 0.16.9` with exit code zero. Only version and help were executed; compilation remains root-owned.

## Existing TeX Bundle

The fullest existing extracted bundle is:

`C:/git/semio/.🧬semio/🦑️repo/⚡️cache/tools/tectonic/bundles/4eaf526eacd253559604ec9a4aaf9b4319abaff487bd7116e611e8537b36a8bf`

It has 502 entries. Its `SHA256SUM` contains the matching bundle identity. Existing `.nx-artifact.json` attributes the finite files to `@semio-tech/print:deps-tex`.

Confirmed resources include `tectonic-format-latex.tex`, `scrartcl.cls`, `tocbasic.sty`, `fontspec.sty`, `tikz.sty`, `tcolorbox.sty`, `needspace.sty`, `l3backend-xetex.def`, `microtype.sty`, `xcolor.sty`, fonts and the TeX base package set. The compiler's help confirms the `--bundle` option accepts a URL or path. Root will establish actual document compilation from this bundle.

The actual executable contains the `TECTONIC_CACHE_DIR` environment key. Set it to the ticket's `🗑️generated/tectonic-cache` to retain format/cache output in the ticket during this rendering task.

## Existing Staged Fonts

The complete preexisting ASCII alias font directory is:

`C:/git/semio/🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/dist/fonts`

It contains the exact names referenced by `semio-fonts.sty`:

| Name | SHA256 |
| --- | --- |
| Anta-Regular.ttf | 9f3b9723a50c1244d49906b1892b3aad43876ab9a2de80288db47821b0c7b298 |
| ShareTechMono-Regular.ttf | 9ceab1f87414829af259c0f537573ae03ef7dd3147c0b27a36a1a0beb6732677 |
| NotoEmoji-Regular.ttf | de6c18832938afc99caf132b39d6a30a19bac7f2e812e28db2535b4608d27551 |
| NotoSansCJKsc-Regular.otf | 2c76254f6fc379fddfce0a7e84fb5385bb135d3e399294f6eeb6680d0365b74b |

A smaller old `C:/git/semio/.🧬semio/🦑️repo/⚡️cache/print-fonts` directory contains the first three aliases but lacks CJK.

No source files or fonts were copied, linked, renamed or edited during discovery. No cache authority, registry pin, runtime dependency, AGENTS file, git state or worktree was changed. All inspection and executable calls used the repo tool interface.
