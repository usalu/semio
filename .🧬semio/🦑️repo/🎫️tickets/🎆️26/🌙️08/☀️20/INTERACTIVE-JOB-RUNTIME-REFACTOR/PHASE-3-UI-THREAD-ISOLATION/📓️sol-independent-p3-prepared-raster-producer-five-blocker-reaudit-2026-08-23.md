# Sol Independent P3 Prepared Raster Producer Five-Blocker Re-Audit — 2026-08-23

## Verdict

**REJECT (source).**

The repaired packet establishes a useful fixed-page producer, a generation-tagged fixed ledger, a
fixed FIFO with checked-out handback, governed one-page advancement, and an OsHost-owned realm-close
cursor. It does not close the admission/hash or exact aggregate-credit blockers for all eight live
Canvas/Paint/Interpreter routes. The permanent 24-mutation predicate accepts those live violations.

No production, test, or verifier source was edited for this independent audit. No build, Wasm,
browser, runtime, network, Nx, or Cargo command ran.

## Audited scope

- Prior independent rejection:
  `📓️sol-independent-p3-prepared-raster-producer-audit-2026-08-23.md`.
- Implementation evidence:
  `📓️p3j-prepared-raster-producer-implementation-2026-08-23.md` and
  `📓️p3j-prepared-raster-producer-census-2026-08-23.md`.
- Live producer/consumer authority:
  `ui/.../wgpu/🦀️prepared.rs`, `🦀️draw.rs`, `🦀️gpu.rs`, product WGPU
  `📦️glue.rs`, renderer `Scenes/🫊️component.rs`,
  `Interpreter/🫊️component.rs`, and WGPU `🦀️os_host.rs`.
- Permanent predicate and its 24 source mutations in root `📜️script.ts`.

## Direct findings

### 1. REJECT — inline SVG still performs whole-source work before reservation

`Interpreter/🫊️component.rs:1688-1692` calls `parse_svg_data_url(src)` and then
`ui_image_digest(src.as_bytes())` before it calls the retained producer through
`resolve_ui_image_svg`. The parser at lines 1631-1642 base64-decodes or percent-decodes the complete
payload into a new owner. The digest at lines 1601-1607 scans every byte of the original source.

This is exactly the prior pre-admission whole-buffer blocker. The fixed ledger is entered only later
at `queue_canvas_image_upload_with`; a saturated producer therefore does not prevent either the
whole parse/copy or the whole digest scan on this live Interpreter route.

The implementation report's statement that Canvas no longer performs either digest is narrowly true
for the helper body, but it does not establish the required end-to-end Interpreter ingress order.

### 2. REJECT — exact aggregate source/key credit is lost before decode

`PreparedRasterReservation::try_reserve_source` initially includes `source_bytes` in the credit at
`prepared.rs:356-368`. After dimensions are known, `claim` replaces that credit with only
`byte_len + key_bytes + page_slot_bytes` at lines 379-398. It does not retain or add the encoded
source bytes.

The live PNG/JPEG path keeps the encoded source `Vec` in the local `RefCell`; the decode closure takes
that owner and the image codec allocates the decoded RGBA `Vec` while the encoded owner is still
alive. The exact resize therefore undercounts simultaneous source plus derived output ownership.
This is not merely an external codec residual: the packet controls and claims the admission ledger
that is supposed to cover those simultaneous owners.

Key ownership is also undercounted. `Scenes/🫊️component.rs:3957` creates a first
`published_key` clone, then `PreparedRasterReservation::finalize` creates another clone at
`prepared.rs:425` while moving the original key into the producer. The first clone is shadowed at
Scenes line 3959 and is not the returned key. `src_key`, created at Scenes line 3884, is retained
through the same interval. `key.capacity().checked_mul(2)` does not prove this actual simultaneous
key-owner census.

Consequently the claimed ordering is only partial: a slot and an initial byte count precede
dimensions; the *exact* source + decoded backing + page slots + all live key owners are not reserved
immediately before the sole pixel decoder.

### 3. REJECT — the permanent mutations false-accept both live violations

The predicate limits its ordering/hash scan to the `queue_canvas_image_upload_with` slice in
`Scenes` (`📜️script.ts:5031-5035`). Its Interpreter check at line 5038 rejects only the former
PNG/base64 roundtrip or missing helper/cache tokens; it does not reject `parse_svg_data_url` or
`ui_image_digest` before reservation. Mutation `hash-scan` at line 5081 inserts a synthetic digest
into the Canvas helper, so it passes as a mutation while the equivalent real Interpreter digest is
accepted.

The credit predicate at line 5024 checks only for the presence of initial source addition and final
`byte_len + key + page-slot` expressions. It does not require source bytes to survive the resize or
count the actual key clone owners. Its synthetic baseline at line 5060 cannot discriminate either
violation.

All 24 built-in mutations reject and the live predicate passes, but those results are not faithful
evidence for the two governing properties above.

### 4. RED evidence gap — advertised hard caps are not exercised end to end

The live constants are present: 16 KiB pages, 16 MiB per item, 4,096 aggregate items, 32 MiB
aggregate bytes, and 256 ledger slots. The exact/+1 fixtures exist in source. The 16 MiB fixture
drives `reservation.claim` but does not materialize the simultaneous encoded and decoded owners.
The 4,096/32 MiB/256 fixture uses a local `PreparedRasterLedger`, not the live ingress/producer/close
chain. Moreover the Scene helper still has a separate 1 MiB decoded-upload cap, so the advertised
16 MiB item boundary is not a live Canvas route fixture.

The fixed 16/+1 FIFO fixture does exercise real producer owners, exact generation identity, and FIFO
order. That positive evidence does not cover aggregate owner accounting.

## Positive evidence retained

- The selected path contains no `pixels[..expected].to_vec()`, Interpreter `pixels.to_vec()`,
  RGBA-to-PNG/base64 roundtrip, `source.split_off`, or producer `source.clone()`.
- `PreparedRasterPages` owns one decoder backing plus page spans; its producer advances one page
  descriptor per call and moves the same backing at completion.
- `PreparedRenderJob::step` checks cancellation/generation/yield, consumes one fuel unit, and only
  then invokes one producer step. The zero-fuel/expired-deadline fixture preserves pointer identity.
- The queue is a fixed 16-slot FIFO. Checkout Drop validates the exact surface/slot/epoch and hands
  the still-resident owner back; stale ABA handback is rejected.
- `PendingRasterAuthorityClose` freezes the admitted surface map, detaches one surface owner, and
  advances page/backing/key/credit retirement incrementally. `OsHostRetirement` drives it after
  frame-build close and before event/presenter/runtime retirement, and terminal requires the global
  map and detached close owner to be empty.
- Exact live census remains three Scene producers and five Interpreter semantic routes: fetched
  bytes, cached SVG, new inline SVG, cached URL-key reuse, and inline PNG/JPEG.

## Residual census

- Infinite World retains the acknowledged `pixels.to_vec()` prepared-raster constructor at
  `infinite/world/🦀️component.rs:139`.
- PNG/JPEG and SVG codecs still materialize a complete codec-owned backing. This audit does not claim
  a streaming semantic codec.
- Atlas/icon/glyph/surface/Vello ownership, platform submit timing, caches, and browser/runtime gates
  remain outside this packet and RED.

## Gates run

| Gate | Result |
| --- | --- |
| Edition-2021 `rustfmt --check` on the seven audited Rust files | PASS, exit 0 |
| `bun ./📜️script.ts verify interactivity --self-test --format json` | PASS, exit 0; 24 prepared-raster mutations execute, broad DENY clean |
| `bun ./📜️script.ts verify interactivity --format json` | PASS, exit 0; one approved process-entry finding, zero unlisted findings |
| Selected exact caller/forbidden/digest scans | PASS as a census; exposed one live Interpreter digest and one Infinite World `pixels.to_vec()` residual |
| Scoped working, staged, and combined-HEAD `git diff --check` | PASS, exit 0 |
| Whole working, staged, and combined-HEAD `git diff --check` | PASS, exit 0 |
| Cargo/Nx/Wasm/browser/runtime/network | Not run by instruction |

The green predicate is not acceptance evidence because its live baseline contains the two violations
above and the relevant mutations do not discriminate them.

## Exact repair packet

1. Move inline-SVG parsing, base64/percent decoding, cache identity, dimension probing, and SVG
   rasterization behind one retained admitted source authority. Eliminate the whole FNV scan; use an
   admitted generation/content authority or incrementally consume one bounded source page per
   governed step.
2. Keep declared encoded-source ownership in the resized credit until the decoder has returned and
   that encoded owner has been cursor-retired. Preflight the exact simultaneous owner census:
   encoded source, decoded backing, page slots, producer key, published key, cache/source key, and
   any codec caller-owned buffer that remains live.
3. Remove the redundant pre-finalize `published_key()` clone and make key construction/publication
   consume pre-admitted fixed key owners. Do not estimate clone multiplicity with `capacity * 2`.
4. Add real live-route boundary/+1 fixtures that hold encoded and decoded owners simultaneously,
   prove exact ledger denial/handback, and exercise the global 4,096/32 MiB/256 registry with ABA and
   close. Keep FIFO identity coverage.
5. Extend the permanent predicate with live-source mutations for Interpreter pre-reservation parse,
   digest, lost source-on-resize, redundant key clone, and every omitted simultaneous owner. A live
   source containing any of those patterns must fail.

Only after those repairs pass a fresh independent source audit should this Canvas/Paint producer
packet be marked accepted. Phase 3 remains RED regardless.
