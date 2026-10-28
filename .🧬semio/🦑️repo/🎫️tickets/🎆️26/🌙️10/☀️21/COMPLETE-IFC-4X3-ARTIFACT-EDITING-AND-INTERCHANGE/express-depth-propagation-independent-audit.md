# EXPRESS Maximum Depth Propagation Oracle

Actual public canonical compiler, maximumDepth0/batch4096, before root's control correction. Authored express-zero-depth-oracle.ts; immutable actual result generated/express-zero-depth-before.json. The script defaults to exclusive-create before output; pass a distinct suffix such as after for a later source checkpoint. No production edits, source copies or runtime installs. Probe exited normally; no validation source mapping remains.

Compilation returns complete=false with827 diagnostics. Rule/function/global expression compilation honors zero depth and emits no records. Other expression categories bypass it:

| Category | Retained records | Retained expression AST nodes |
|---|---:|---:|
| derived |60|346|
| UNIQUE |4|9|
| contextual bounds |6|6|
| inverse declarations |165|316|

Inverse158 declarations contain parsed bounds;7 have no lower/upper expressions and may legitimately remain as metadata under a zero expression-depth budget. The bound-bearing158 should not acquire AST entries by bypassing the supplied depth. A per-job compiler may refuse those jobs entirely, preserving its ordinary partial-program inspection semantics.

Examples: IfcAxis1Placement.Z and IfcAxis2Placement2D.P remain call ASTs; IfcApplication.UR1/UR2 remain aggregate ASTs; IfcBSplineCurve.ControlPoints.upper and IfcBSplineSurface.ControlPoints.upper remain identifier ASTs. These are actual released declarations, not synthetic programs.

Independent direct parser control: new ExpressParser('1',0).expression() throws ExpressSyntaxError depth exceeded at0. Existing parseExpressExpression('1',0), invoked through JS's extra argument behavior, accepts because the helper's declared signature ignores the second argument and constructs default-depth256. Therefore forward captured depth to every helper and every compile job category, not only rule/function/global parsers. The before result is a resource-control finding, not evidence of a wrong EXPRESS schema predicate or native execution.