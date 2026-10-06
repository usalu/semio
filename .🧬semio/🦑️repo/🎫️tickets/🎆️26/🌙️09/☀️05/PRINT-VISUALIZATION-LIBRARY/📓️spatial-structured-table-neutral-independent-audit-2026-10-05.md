# Structured Spatial Table Neutral Audit

Current six authored controls cover polygon/line/point/grid/points/vector-field, each EN/de and both themes after actual mutation/replay, plus a constant-route normalization control. Before schema/native feature patches, these are RED inputs, not completed behavior.

Concrete harness concern: record normalization invokes value.replace despite ProbeRecord.values being number|string (existing viz-probe.ts9/78). Actual research probes return numeric values. Normalize with String(value) before string whitespace removal.

D3 nested interpolation expectation for mutated vector corners is correct: u=(5-1+1+3)/4=2 and v=(0+2-2+0)/4=0 at midpoint0,0. Manual post-render vector_sample validates registry/interpolation, but does not independently verify named field reaches quiver/streamline rendering; use actual renderer sampler/arrow probes for this integration boundary. Grid normalization expected values4,1,2,3 against domain1..4 is independent and correct. Geometry and point sequence path reconstruction through D3 line preserves authored order.

Launch/seed retained full snapshots are pure416-byte insertions with identical block parity and no outside-byte change. However both current spatial-data blocks use order900.037127, duplicating geo-palette, rather than intended900.037128. Root notified to correct the existing new block before freeze.

Body post-render geometry probes query global stores correctly. Grid post-render load is an explicit registry probe, distinct from actual rendered cell/path verification. PDF.js currently consumes every operator list; this is parse admission, not text/glyph checks or physical geometry comparison.

🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts — 3F3E080283868E18D0302C4133843A8B81AF0D7976F6AC535F2BDE7A6D846534
🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🔣️.json — 5F961933333DB9E6D6AFD29C97A17E3E5790E7C0A4173C7A26A03BAC6A3DD4AA
.vscode/launch.json — 1C0E438FD306546F1CEF64F363518F462C711634F82B949E1753CB93B89D8558
.vscode/🧩️launch.seed.jsonc — 32F0A7DC9162BC0CF07D705916886E29310C0E1F9D2CF90B8E1AA1012D0005BC

Root corrected String(value) normalization, added actual georte_sample wrapper, and both phase order128. Retained order before/after full-byte reversal (128→127) passes both launch files. Geo normalization guard replaces eager fp ternary with expandable fp_compare before division, so collapsed extent returns0.5 without evaluating0/0; full before/after diff is confined to that guarded expression.

New renderer oracle boundary: native arrow_at501–505 samples x=column*0.7−3 and y=row*0.7−2. For2×2 controls these are−2.3/−1.6 and−1.3/−0.6, not vector fixture domain corners±1. Comparing actual outputs by row index to corner u/v requires a deliberate domain-mapped named-field lattice change; otherwise use captured sample coordinates and independent clamped bilinear expectation. Root notified before product candidate.

Root clarified named-field renderer design deliberately maps lattice indices to declared vector domain, preserving analytic sampling coordinates; under that future implementation the2×2 row-major corner oracle is intentional and appropriate. Schema structures use explicit column references rather than heuristic names. Prepared TS spatial emission checks declared finite numeric columns, consistent geometry group identity/value, vertex count, full rectangular grids with duplicate rejection, uniform complete vector axes with duplicate rejection, and checkpoint calls while traversing rows/cells. Local generated geometry point arrays mutate only their own closing duplicate, preserving authored rows.

Current helper reread still opens spatial-data.pdf under each appearance directory; Root reported actual appearance.pdf basename. This unresolved source/receipt discrepancy was sent for confirmation before finalGREEN. No additional concrete candidate semantic mismatch identified beyond the recorded helper boundaries.

Prepared native vector store mathematical review: row-major index y*w+x+1 is consistently used for the four bilinear corners. Continuous scaled x/y clamp to0..w−1/h−1; base cell caps at w−2/h−2 so max-edge fractions1 select the last sample without an out-of-range neighbor. Midpoint2×2 gives equal quarter weights; strictly ascending declared domain and dimensions>1 exclude division by zero. Reusing a named store updates sequences/dimensions/domain; selected name/domain variables are separate from generic scratch values.

Existing source already uses seq_count:c/seq_item:cn/clist_gset:cn; fp_compare_p:n occurs in current scientific/chart consumers. Candidate clist_set_eq:Nc and full predicate spellings still await actual candidate native compilation rather than an inferred macro-existence PASS. Low-level declaration writes store state before its native validation diagnostic; canonical structured TS inference validates first and publishes atomically, so direct macro error semantics remain a distinct scope. No runtime numeric/vector PASS is claimed for this unapplied candidate.

Existing Rust spatial candidate bounded review: declared-column finite extraction, typed geometry groups/value consistency and vertex validation, checked complete rectangular grid with duplicates rejected, uniform sorted vector axes and row-major cells align TypeScript structures. Paid merge sort checkpoints writes/comparisons and subsequent cell validation; no concrete multi-implementation mismatch found in inspected ordinary finite controls. Actual structured22/native gates remain required; current typed Rust native26016 independently reported terminal0 is a separate58-option/28-source gate.

## Named vector renderer candidate source review

The retained `native-vector-renderer.candidate.txt` has SHA256 `73947501E6BCD3FC0E0C1DB4AF8F391928389A1C7281B2E7816FC54F1432E376`. Its quiver loop (505–511) explicitly starts both row and column indices at zero. Consequently the named-field mapping at 519–520 reaches both declared domain endpoints when the respective lattice dimension is greater than one; a 2×2 lattice samples the four corners in row-major order. The denominator guard maps a singleton dimension to its minimum endpoint. There is no one-based offset discrepancy.

The named streamline branch seeds within the declared domain, maps domain coordinates to the same zero-origin lattice extent used by quiver, and clamps integrated state to the declared domain. RK4 retains the four weighted samples and final dt/6 update (574–624); its intermediate named samples use the store sampler's clipped interpolation coordinates. The analytic branch retains its prior sampling, seed and clamp constants. This is a bounded source review of a retained candidate, not proof that the candidate has been applied or passed native compilation. The planned named-streamline case is needed to bind renderer execution independently of standalone registry interpolation.

Independent current-disk comparison of spatial-rust22-seven-final-source-bindings.json checked all six paths with zero mismatches. This binds schema54DEC, RustA9B36, testF1F070, fixtureB9EE6 and the two remaining declared inputs to the pending17224 producer. It does not replace terminal/runtime evidence. Detailed comparison remains generated/closure-spatial-rust22-seven-source-rehash.json.


After producer17224 actual terminal0/all22 reported by its owner, independent disk verification checked all28 unique output paths against spatial-rust-seven-final-output-bindings.json: zero SHA256 or byte-count mismatches. The seven scenarios each have explicit en/de and light/dark output carriers. Native Rust consumer68915 remains pending, so this proves emitted carrier binding only.


Independent first-party context check rehashed all826 unique paths in spatial-rust-seven-firstparty-input-closure.json against current disk with zero missing/hash/size mismatches. This49-package/50-dep-info context is explicitly build-input evidence, not an ownership claim; none of its additional context paths is automatically added to ticket close files. The snapshot binds producer17224 and has SHA9869E8EC743E1A6E3D2F2CBAD95CF239FA4D0BF7E99682E46F4718BE5D9A36AA.


Rust native consumer68915 is actual terminal1. Its light document compiled, but the first rendered vector-field-en sample was [-3,-2,-0.9092974268256817,-0.1411200080598672] instead of independent [-1,-1,5,0]. This is concrete consumer evidence that the named renderer path was not reached for that executed carrier; a correct direct registry sample alone does not prove family integration. Native confirmed current production geo-routes1C1391CE differs from the retained candidate73947501 reviewed above. Stage/current source and caller token lifetime must be traced before attributing the failure to either emitter or renderer. No additional job was started by this audit lane.

Independent source read supports Native's token-category diagnosis: data.sty vector_new743–747 and vector_use779–781 normalize names with tl_to_str:N before registry membership, whereas the routes family membership uses the authored field token list directly. Equal visible names therefore have differing letter/other token categories. The emitted field option is present; this failure is a native registry/renderer admission boundary, not an absent Rust option. Source-owner repair and actual native replay remain required.


The released named-field lookup patch independently reverses exactly: removing its two identical field-TL `tl_to_str:N` normalization lines reconstructs the full retained before file byte-for-byte. Both insertions precede registry membership, making the family and subsequent sample lookup agree with registry token normalization. No other source span changed in these snapshots.

Current full helper invokes the spatial controls directly, and also emits Rust spatial carriers in its existing Rust producer environment before consuming those carriers through compileNativeSpatialDataGrammar. The seventh oracle requires twelve actual renderer sample records and three integrated states; D3 bilinear interpolation independently computes four RK4 stage values per step, dt=.2 weighted updates and domain clamps. Its initial state is taken from the first actual sampled seed, so this oracle proves field interpolation/integration/path projection from that seed, not independent random-seed placement. This scope does not weaken the independent component expectations or the required named-renderer sample inventory.

Current configured-host evidence EAE61438894C796B7462D25D24B02F15A4C05E3E45D767EEFD7082DC3AC83B32 independently matches its disk SHA. All29 unique sources in its sources array rehash with zero drift. This binds the new reported30212 all-three-host terminal to current29; no broader pre-gate freeze is inferred because API/metadata remains open.


Worker30212 binding is superseded for current schema: Native's fresh check reports28 unchanged inputs and schema54DEC→045A as one delta. The earlier independent zero-drift observation remains scoped to its check time; final stable-schema configured-host replay remains necessary.


Choropleth palette consumer repair independently reverses exactly by replacing its three palette_steps references with the prior steps spelling; no old geo_f_steps_int reference remains under latex. For paletteSteps1 the default sequential legend is safe: ramp_quant's integer guard returns59 without evaluating the supplied0/0 token expression. A separate concrete source boundary remains: diverging ramp:n evaluates fp_compare on that expression before invoking the guarded quantizer (geo.sty2464), so a one-level diverging legend cannot rely on that lazy guard. Root notified; the sequential runtime control does not claim diverging coverage.

Actual geo transform1089–1104 explicitly implements D3 scaleTranslate: x=tx+k(px−ox), y=ty−k(py−oy). Therefore local geometry probe coordinates must compare directly to D3 projection output; adding30−D3y is an extra inversion. This local API convention is separate from the PDF page CTM and does not establish a general screen/north-up rendering claim. Root89285 failed this newly authored inverted oracle after successfully compiling one-level sequential choropleth; runtime coordinate source correction is not warranted by that failure.


TS42628 binder exactSHA5E51CC matches; independent disk verification checks756 artifact records,16 viewed-page images and2 executed retained inputs with zero SHA/size/missing mismatches. Executed helper7F16 and fixture23B retained snapshots match. Scope remains historical exact staged66D/B27B/434 and light document theme, even where palettes vary. Prospective document-theme helper patch independently reverses exactly by removing theme=+appearance in precisely2 configurations; no other helper bytes changed. It does not retroactively make42628 a dark-document proof.


Final broad94896 prospective source capsule765 rows independently rehashed0drift (capsuleSHA50512F739E39A0CD25629C8B6D0329E333694709AE79019AE4539D33F8F946AD). Separate826 Rust context capsule independently rehashed0drift (SHAABCE79660AC2C1D7BD7881DD71D338714107ADCF2CC850CB2AD447BFFDFAB8DF). Both are retained under authored-inputs/final-native-grammar-current045, not generated. These are explicit prospective context/input bindings, not automatic ownership of all listed paths. Final compiler294 snapshot also rehashed0drift at current check; terminal full publication/consumption remains pending.


Current broad94896 completed TypeScript spatial stage binder8285CAD independently matches its diskSHA. All740 artifacts and16 actually viewed image rows rehash with zero byte/SHA drift. Fixed2272495-byte live-log prefix independently matchesF2EAC1622D4D0DC1E69A556B69706F5C2D423360CDAAD72DD7CF5874CFCC72AC using shared-read; mutable whole-log hash is not substituted. Stage binds prospective765 and actual enclosing light/dark documents. Parent94896 remains live. Final045 source-status capsule29 also independently current-disk0drift.


Independent completed Rust producer/stage binding checks:28 carriers zero SHA/size drift;826 Rust context inputs current0drift;172 current native sources0drift. All1548 source file instances across9 actual staged libraries match, allowing only the declared graph shim reference normalization (F33D5F16). The graph implementation sits in sibling .semio-library/modules/graph/_.tex and all9 instances match81BE72 exactly. No unexplained source mismatch found. Parent94896 remains live; these checks do not mark its full terminal complete.


Independent final Rust completed-stage check: the740 artifact hashes previously checked remain bound to the retained D8A842 binder. Its fixed2320776-byte live-log prefix independently matches738A0A945F2E6F2BA448BE053B1A9BA1FA0121AA2F905C67526E1B3ECDF4AA61. All four actual TS/Rust light/dark PDF hash and byte-length checks match their recorded pairs (light3AB1DB58, dark96671981), so the16 actual TS reviews transfer by identical whole PDF bytes. This is completed spatial-stage evidence only; parent94896 remains pending.

Full94896 authoritativeRoot terminal0/77m36 now independently bound: completed log2460008 bytes SHA6386A062235A52D97D513FAC8C0030E31F811C63E4381BF6869CEC036A7EB8C3 matches. All765 current adjudicated product hashes and826 current adjudicated Rust-context hashes match disk0drift at this check. Against original prospective captures,765 has6 declared differences (5 authorized API-document/verification paths plus concurrent launch registration);826 has1 concurrent cargo-config context change. Therefore this is not blanket unchanged prospective-source proof. Native option/admission Rust-specific final controls remain separately refreshed; final catalogue13588 consumer pending.

Independent terminal94896 native-stage audit now rehashes all71 libraries ×172 mapped files =12212actualsource instances and71 graph implementation files. Every authored source matches the original prospective native hashes; only registered graph shim normalized-path hash is used for that one mapped file.0unexplained differences. Compiler-generated texput.log is excluded as output. This independently supports the retained terminal-stage-source-bindings checker; no source or stage mutation.
