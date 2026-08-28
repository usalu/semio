# Canonical Return Result Framing

## Purpose And Source Ownership

The private raw response handoff cannot charge only its 4161-byte backing while silently creating a second decoded ActorBytePage. The runtime coordinator accepted bytewise canonical result framing, using the same return grammar, tags, enums and receipt identities. This source is a public format parser, not a resident lease, captured producer, UI owner or InputAck authority.

The declaration is actor/📤️return/📄️framing/{🧬️schema.json,🧪️fixture.json}. The implementation is colocated with the existing return codecs in actor/📤️return/🟦️component.ts. ActorReturnResultFraming.push(byte) consumes exactly one byte and keeps only a fixed set of metadata scalars. It does not allocate a payload array or construct a page record. finish() validates terminal framing and yields a stable frozen ActorReturnResultProjection. A page projection contains only kind, exact receipt and payloadOffset relative to the result start. The caller retains the actual original backing and must admit parser/metadata owners separately.

The parser enforces canonical ULEBs, positive exact authority, safe request numbers, 0..4096 length, boolean/finality rules, all enum subsets and control-success/fault consistency. Page padding is checked one byte per call. Oversized, malformed, trailing or truncated input makes failure sticky and cannot expose a successful value. Offsets and format-valid fields do not mint input authority.

The existing whole decoder now consumes this one framing parser and explicitly materializes a page only for callers requesting the whole decoded value. It remains a whole conversion and is not the private retained runtime path. No compatibility format, duplicate wire layout, guest API change or generated output was added.

## Executed Tests

The two authored preimplementation tests actually failed at the absent constructor: 0 passed / 2 failed / 129 skipped, 131 collected, nine files, 1.21 s, start 22:49:39. This is missing-parser TDD, not a native ownership failure.

Focused GREEN1 actually passed both tests / 129 skipped / 131 collected, nine files, 1.84 s, start 22:51:09, exit 0. Strict Ajv validates the neutral declaration and page projection. Existing shared result/page vectors are reconstructed independently with webassemblyjs LEB128 and Node Buffer. Empty and full 4096-byte pages are fed bytewise while Uint8Array, BigUint64Array and ArrayBuffer constructors are instrumented: no payload backing is allocated by the parser. Tests compare exact payload offsets, frozen/stable results and all fixed variants, and reject contradiction/truncation/trailing/padding/invalid-byte cases with sticky failure.

Full actor actually passed 131/131, nine files, 6.65 s, start 22:52:15, exit 0. Strict reported exactly seven tutorial diagnostics, exit 1, with no return/input/parser errors. The full target also exercises the existing canonical codec's enum-combination and every-prefix truncation cases against the unified parser. This packet does not prove host allocation admission, exact one-response credit, raw-root retirement, native guest execution, browser timing or six-app content.

Released SHA256: return codec 87c7f25b1aed9bbc15bc3916d837bdd518140bec7e93bd04ba3eac1831edd59f; framing schema 5a3a0d87b73257932d0b1e73f1d5cf1ad0144e6d869f3eddae614e480290fc24; neutral fixture ab7e908a44fa04375d16b9a5163d62980c6e7166a04601c99c0a44adf42ed5d5. The runtime coordinator received this exact source/API release. Dedicated focused launch registration was requested from the taxonomy owner; existing ActorReturnResult and full actor targets already collect the cases.

The runtime coordinator independently executed actor R19: 131/131, nine files, 4.36 s, start 22:54:17; OwnedPaged R3: 15 passed / 643 skipped / 658 collected, 11.46 s, start 22:54:19. It reported all 31 selected TS/config/JSON inputs stable, including this framing declaration and the renamed credit packet, and released every source hold. Its subsequent strict R33 reported exactly seven tutorial diagnostics. These are delegated independent results, distinct from this ticket's own runs and still not live guest proof.

The coordinator subsequently reported full React R24: 658/658, five files, 160.83 s, start 22:57:53, exit 0, with all 55 selected TS/config/JSON inputs stable. Its report is in the runtime ticket at 📓️coordinator-renderer-react-full-r24-2026-08-27.md. This larger regression boundary does not change the unimplemented raw-slot or native/live runtime status. No source hold remains.

## Simultaneous UI Join

The UI owner added one separately charged 128-byte observation turn after each source field child step, plus private reader installation. Peer success tests now derive their loop bounds from the actual phases: copy uses payload bytes plus three transitions per destination page plus five fixed input/proof transitions; continuation uses one source step and one observation per actual field step plus one ready observation. A partial page uses three turns per copied byte plus page/fixed transitions. No runtime cap or resource quota was raised. The exact original field remains the only source of offsets and completeness; only its bound UI builder drives it.

All active-ticket evidence, logs and compiler caches are preserved. No native compilation or catalogue/worker/Wasm publication was performed.

## Next Integration Dependency

The shared resident ledger owner has published and begun implementing the neutral page/owner API, but the exact raw external-backing slot and private transport-close proof are not yet released. The complete proposal was read in the runtime ticket's shared-resident-ledger API report. This ticket sent concrete raw-slot names for review: reserveExternalBacking, beginReceive, adoptTransferred and private exact storage receipt; no speculative runtime implementation or structural finalizer was added. A posted slot without a response cannot be refunded by cancellation or by a caller-provided boolean. The next safe mount requires that agreed private storage/transport handoff.

The demonstrator goal remains incomplete: neither fresh native publication nor real six-app content/interaction/close validation has run at this boundary. The active ticket stays open, and all peer caches/evidence are retained.

Taxonomy reported collision/no-follow checks and canonical seed registration for focused framing 400.92 and response-credit 400.93. Generated launch publication is held while the runtime lane's existing output-only resident gate 408.32 receives its stable seed join. Every row is preserved; neither new command was executed by taxonomy. Final generated-output hashes are still pending, so registration is not described as fully published here.
