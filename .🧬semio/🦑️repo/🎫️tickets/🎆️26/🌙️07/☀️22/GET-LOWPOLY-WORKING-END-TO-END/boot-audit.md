# Boot Path Audit

Node path.resolve and fs.existsSync verify that the publication source imports using four parent segments resolve to C:/git/semio/🧰️framework. Five parent segments resolve outside the workspace and do not exist. The publication ScriptRouter project root resolves to ✏️s/🧑‍💻dev and its repository root resolves to C:/git/semio. No path changes are needed in the audited publication source or command router.

The publication tests use five parent segments because they live one additional directory deeper; those workspace roots are consistent. Their generated projection root resolves to catalog/🤖️generated.

At audit time the generated catalog directory was absent. Consequently no current generated Lowpoly host command could be extracted. The authored generation target is repo run @semio-tech/s-dev-composition:generate and invokes repo ./🧩️catalog/📣️publication/📜️script.ts generate from ✏️s/🧑‍💻dev. It takes no direct arguments. The command first depends on generate-stdio-format-claims.

Smallest coherent fix set for the audited paths: no changes. Boot diagnosis must continue at catalog generation and the inferred host target rather than changing import depth.
