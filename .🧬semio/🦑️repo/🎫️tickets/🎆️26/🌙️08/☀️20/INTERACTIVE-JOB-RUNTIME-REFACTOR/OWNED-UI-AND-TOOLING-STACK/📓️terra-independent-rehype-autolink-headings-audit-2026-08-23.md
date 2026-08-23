# Independent `rehype-autolink-headings` Retirement Audit — 2026-08-23

## Verdict

**ACCEPT**

Blockers: none. This is a narrow dependency-wave acceptance only, not Phase 10 acceptance.

## Independent Evidence

The live diff contains only the prescribed removal of the root Storybook import and sole `rehypePlugins` item, root/UI direct manifest rows, and two root/UI Bun workspace tuples. No guard, Compose, Dagre, MDX Rollup, Storybook glob, externalization, P3/P8 source, shim, fallback, facade, or replacement dependency was added.

Fresh uncached `bun x nx run @semio-tech/ui-react:build --skip-nx-cache` was a cache miss with Nx status `0`. Its permanent guard/index result is exact: **231 = 170 stories + 61 docs**, 61 unique `.tsx` inputs, 61 `.tsx` Autodocs entries, zero MDX/unsupported inputs, and SHA-256 `72e76f1580736f6612ed36b57d8fee1b0461adf1bc9c3c25ab88fe9e83713ce4`, equal to the reliable executor pre/post hash.

The owned non-Compose/non-ticket MDX census is `0`; static, dynamic, and CommonJS `.md`/`.mdx` module-edge scans return no matches. Installed `@mdx-js/rollup` invokes processing only for Markdown/MDX extensions. Installed Storybook core creates Autodocs by retaining the CSF `importPath`; the separate `extractDocs` method reads actual MDX, while Storybook's own MDX plugin filters `.mdx` and is removed by root configuration. Generated Autodocs therefore cannot reach the removed HAST anchor transform.

## Gates

| Gate                                             | Result                                                  |
| ------------------------------------------------ | ------------------------------------------------------- |
| UI quick                                         | PASS; 21 files, 724 tests (retained capture below)      |
| UI lint/typecheck                                | PASS                                                    |
| Frozen install and ratchet                       | PASS; 131 current, 107 removed, zero new                |
| Direct lists                                     | PASS; 68 JS without target, 63 Rust                     |
| JS parity                                        | PASS; 83/248/104/144/0/44/0/5                           |
| Root/UI absence                                  | PASS                                                    |
| Compose retention                                | PASS; 3 manifest/lock tuples and one `7.1.0` resolution |
| Root script syntax                               | PASS                                                    |
| Scoped and whole working/staged/HEAD diff checks | PASS, all zero                                          |

`@mdx-js/rollup`, `dagre`, and the permanent guard remain active/unchanged. Shared Prettier baseline is the pre-existing `.storybook/main.ts` and concurrent root-script drift; changed manifests pass. The formerly stale prompt-index whitespace was synchronized before final verification: all three whole-tree checks are clean.

## Retained Artifact And Audit Scope

The independent quick-test capture is retained at `🧪️terra-independent-rehype-autolink-headings-quick-2026-08-23.txt`. It was initially created at `/tmp/semio_autolink_quick.out`, then relocated unchanged into this ticket folder after the artifact-location violation was identified. A subsequent `/tmp/semio_*` search returned no remaining files.

No production, manifest, lock, Git, ticket-lifecycle, or Cargo change was made by this audit.
