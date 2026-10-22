# Captured CommonJS Boundary Contract Decision

The actual a668 startup failed because the Bun workspace facade imported a named export from captured CommonJS. The authored CommonJS file sets module.exports to an object; the captured wrapper exposes that value as its default export. The current peer facade now imports the default value and explicitly exposes the existing first-party factory.

Adopt that canonical default-value boundary. It gives the facade direct control over the public interface and avoids reliance on heuristic synthetic named exports. This preserves the existing public first-party function contract and adds no compatibility layer. A generic named-export parser, namespace bridge or loader fallback is outside the concrete repair needed here. High will preserve peer changes and integrate their actual captured factory regression, adding only missing evidence. Low reviewers independently check captured behavior and the closed language-agnostic fixture/owner-member pairing.

Historical a668 facade bytes and current peer bytes are separate observations. The post Source schema552-byte CommonJsBoundaryCase addition is regression DATA; it neither repairs runtime exports by itself nor identifies historical loaded code. No Native process is live or authorized to retry. A future policy must use285 as its old critical roster, matching the last actual attempt.

## Genuine Regression Scope

The added case will import the actual workspace and payload facades through captureCompilerBuildSourceClosureV1 and the existing immutable owned loader, then call real parseBunWorkspaceDeclaration and ownsPayload. Neutral case outputs will be compared with a SQLite oracle. Existing peer AST and synthetic-factory laws stay in place, with explicit finite ceilings. The schema will bind the exact owner/member pairs and the AST reader will use the physical Source origin. High is authoring this case; no runtime pass is claimed.
