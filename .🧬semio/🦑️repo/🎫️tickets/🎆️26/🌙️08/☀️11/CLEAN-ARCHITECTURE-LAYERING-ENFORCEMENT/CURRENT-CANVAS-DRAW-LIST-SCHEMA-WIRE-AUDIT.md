# Canvas Draw List Schema Wire Audit

Read-only actual encoder/schema audit. New General source not yet moved at inspection; actual encoder authority remains OSInfinite Canvas renderer::draw_list. Schema is variable production command data, not a fixed fixture/testplan schema.

## Confirmed mismatches

push_path_elements writes flat `['p',[0,x,y,1,x,y,2,cx,cy,x,y,3,c1x,c1y,c2x,c2y,x,y,4]]`. New schema PathVerb expects arrays inside the path stream. Actual bounded Ajv strict probe rejected a valid flat MoveTo/ClosePath `['p',[0,1,2,4]]`. Existing graph fixture uses primitives and may not exercise this branch, so its schema green cannot close paths. Preserve flat wire with numeric-array schema plus semantic opcode/arity validation at runtime or deliberately redefine all encoder/replayer consumers/laws together. Draft07 tuple schema alone cannot describe arbitrary repeated variable-width flat verbs.

Image base64 regex admits noncanonical trailing bits. Actual Ajv probes admitted both Zh== and Zm9=. Canonical final2-padding quartet requires second sextet [AQgw]; final1-padding quartet requires third sextet [AEIMQUYcgkosw048]. Use these restrictions along with exact4 grouping, alphabet and EOF anchors; empty remains valid. Decode/reencode through existing strict Base64 and Buffer oracle provides semantic tests. Image byte length versus dimensions remains a semantic constraint, not presently schema-validated.

## Matched actual output branches

Fill f5 fields, Stroke s9 fields, Image i5, PushLayer pl6, PushClip pc4 and Pop po1 match actual encoder. Stroke contains width/startcap/endcap/dash-pattern/dash-offset/RGBA/affine/shape; join/miter are not transmitted, so schema must not invent them. Rule0..1, cap0..2, blend0..15 agree. Shape rect5, roundedrect9 in clockwise TL/TR/BR/BL, circle4, line5 and cubic9 agree. Affine6 agrees. Color to_rgba8 uses f32*255+0.5 cast-to-u8 saturation and emits four integer bytes; alpha in pl is rounded numeric f32 via push_number rather than RGBA alpha conversion. push_number replaces nonfinite with0, rounds configured decimals and trims trailing zeroes. Schema Number permits actual finite output and does not enforce an arbitrary quantization/default-decimal constraint.

VelloFragment currently encodes po despite not being a pop; schema cannot prove replay equivalence for that opaque branch. This is existing behavior to preserve/audit separately, not new schema authority. truncated only compares scene command count versus maximum_commands. No schema bound on command count proves an expensive-operation bound.

## Law and oracle qualification

New ownership law uses independent BunTOML/Iarna for manifests and Ajv for production schema admission. JSON.parse/JSON5 comparing the same expected JSON proves parser agreement only; it is not an independent draw encoder. Original native scene law paints the neutral graph corpus and compares to expected drawing; JavaScript Flow twin replays the same output into actual2D command spies. Preserve both laws and source path rebinding. Add neutral path all opcodes, arcs, image remainders/canonical low bits, alpha rounding/nonfinite and each command shape admission/refusal cases. Existing base64 and renderer color differential laws supply independent third-party byte/color engines; do not add fixed corpus schemas as authority.
