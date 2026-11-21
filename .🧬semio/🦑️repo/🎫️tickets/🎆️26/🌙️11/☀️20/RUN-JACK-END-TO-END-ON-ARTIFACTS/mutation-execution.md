# Jack mutation execution

The synchronous runner and retained query job use the same evaluator. A MATCH extends incoming rows. CREATE and MERGE bind their pattern variables, reuse bound endpoints, and allocate free identifiers. Deleted identifiers stay reserved until the query finishes so an earlier binding cannot resolve to a newly created entity. SET and DELETE operate over each incoming row and accept both nodes and edges. RETURN reads the working graph after the evaluated mutations. An edge-only graph result includes its endpoints.

The evaluator validates each effect before changing its working graph. The synchronous effect applier journals the original entries it touches and restores them if any effect fails, including a deleted node's incident edges and root selection. An edge endpoint can identify a node directly or identify an existing port on that node.

The resumable-query fixture checks created and matched bindings, multi-row updates, edge writes, cascading deletion, and refusal after a valid mutation. The SQLite oracle passed through the pinned Bun runtime. The Rust regression for a failed effect batch compares the entire graph before and after rollback.

The checkout's job API now receives original Work effects for retirement and uses a return origin to receive job outcomes. The Jack query close cursor and the framework layout testing driver follow those contracts. The native executor build passed the layout testing dependency and reached the Jack inference dependency, where checkout-wide changes to binary receiving and retirement were still in progress.

The parent `SetRoot` operation supports selecting an identifier and explicitly clearing it. Its inverse captures the prior parent value, and its sparse diff preserves the child address. This lets the editor clear the selected root in the same edit as a child node deletion. The parent binary operation uses the controlled receiving codec.

The authored root-clear fixture has TypeScript, SQLite, Ajv, Python and Rust checks for the result and inverse. The three TypeScript tests passed. The native check compiled the parent mutation and receiving implementation and reached host retirement API changes.
