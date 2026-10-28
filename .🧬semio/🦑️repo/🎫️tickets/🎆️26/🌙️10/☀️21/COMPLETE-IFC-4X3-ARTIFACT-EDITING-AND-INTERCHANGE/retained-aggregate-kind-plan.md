# Retained Aggregate Operand Classification

The targeted independent audit confirms two related source defects. Scalar + right SET builds a SET but chooses deduplication from a synthetic left LIST; 'INTEGER' + TYPEOF(1) therefore predicts a duplicate INTEGER. BAG * SET chooses BAG from the left input, and a subsequent union makes that wrong result kind observably retain duplicates.

Closed neutral operand/result vectors, TS reference cases, native original-machine tests and an independent IfcOpenShell express_set test interface precede production changes. The correction must use actual operand/result categories before scalar promotion. Test both operand orders, BAG/BAG multiplicity, SET/SET uniqueness, mixed intersections and a nested union before any function return coercion can mask the bug. All new matching/contains work retains the existing explicit continuations and original five-axis loans; no new graph/source owner or recursive evaluator is introduced.

TYPEOF containing-SELECT closure remains a separate normative candidate. Both current TS and the inspected IfcOpenShell helper omit it; a pinned schema SELECT roster proves membership but not full TYPEOF semantics. No closure behavior is changed on that evidence alone. Unknown aggregate output membership continues to refuse explicitly. Template semantic admission/native execution flags remain false.

The guided staged-template form plan has been written, but all form/framework implementation remains paused until this semantic fix checkpoint is independently reviewed. Canonical AST arrays and scheduling indices remain unchanged.
