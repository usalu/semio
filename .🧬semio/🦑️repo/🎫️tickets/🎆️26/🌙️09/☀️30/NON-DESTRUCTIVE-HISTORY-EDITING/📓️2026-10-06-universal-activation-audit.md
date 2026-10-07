# Universal Live Activation Audit

Source-only audit on 2026-10-06. No live pass is inferred from source or module-import tests.

The universal browser journey keeps the complete `--serve` URL as its route, while using the origin for the local server. Existing servers are reused by the shared server fixture. The server fixture does not verify or replace another running server's catalog and never stops a reused server.

However, `runTimeTravelCli` still requested the literal puzzle2d startup variant for a missing server, including a universal route aimed at another family. A family playground session contains its owner and dependency closure; a puzzle session cannot be assumed to contain unrelated editors. This makes automatic startup of a non-puzzle universal route incorrect.

The UI execution owner is correcting universal startup to use declared playground metadata or an explicitly validated variant, with a neutral fixture law. The normal puzzle-specific acceptance route retains its existing declared puzzle variant. Native and React activation must finish before live probes can be claimed, and a listed prompt choice alone is insufficient: the universal journey now performs both overwrite and named-alternative outcomes.

Relevant source owners: plugin registry playground session `buildPlaygroundSession`; dev local-hub execution `ensureDevServe` and `devServeCommandV1`; dev time-travel probe `configure`, `routeUrl`, and `runTimeTravelCli`.
## 2026-10-06 15:15 — Startup Repair Source Proof

UI implemented manifest-driven universal startup: it selects the declared route's plugin/alias, an explicitly validated variant, or a unique renderer port. Conflicting, unknown, and unclaimed choices are refused. The native canonical plugin route is retained, while the ordinary puzzle journey keeps its declared puzzle variant. The neutral Ajv/JSDOM/WHATWG URL and module-load cohort moved from four failing/four passing cases to twelve passing cases. This verifies startup selection logic; live universal editor operations remain pending actual activation and browser runs.
