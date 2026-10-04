# Chart Native Raw Value Equality Physical Owner And Controls

Read-only actual APIs/laws; no executions. `DslValue` at Framework Value `🦀️.rs:141` derives PartialEq over actual variants Null/Bool/Number/String/Bytes/Array/Object. Arrays and Object's Vec of key/value occurrences are ordered and preserve duplicate occurrences under this equality. It is stronger than current Print chart_values_equal, which searches object keys and converts every number through as_f64.

Raw DslValue PartialEq still does not prove exact numeric native identity: Number::UInt and nonnegative Number::Int can compare equal; Number::Float uses float equality, collapsing positive/negative zero and refusing NaN self-equality. Float versus integral variants compare unequal. Exact Native persistence demands should compare Number discriminants and Float.to_bits, ordered key occurrences and recursive full values, or independently assert raw-word/variant fields. Comparing only ChartSnapshot's custom PartialEq or serde JSON cannot prove those properties.

Actual physical writer/reader signatures (Framework IO `🪶️sqlite-snapshot/🦀️.rs:294,299`):

```rust
pub fn export_sqlite_database(database: &SqliteDatabase, limits: SqliteDatabaseLimits,
    callback: &mut dyn FnMut(SqliteSnapshotProgress) -> bool) -> Result<Vec<u8>>;
pub fn import_sqlite_database(bytes: &[u8], limits: SqliteDatabaseLimits,
    callback: &mut dyn FnMut(SqliteSnapshotProgress) -> bool) -> Result<SqliteDatabase>;
```

Existing actual physical unit laws (`🧪️tests/🔬️unit/🦀️.rs:66–73`) demand file/row/table/column/value refusal and real page-boundary cancellation both directions; independent Bun SQLite at90 onward verifies physical readability, joins and foreign keys. New allocation laws176/181/187 cover cross-stage frontier, failed-stage retained backing and real native bridge interior failure settlement. These are suitable existing physical authorities, but Chart still needs its own complete domain query/edit/schema+native-control witnesses; this audit does not say those laws passed in the current source epoch.

Native Decode and Encode controls live in Framework Value `🛬️decode/🦀️.rs` and `🛫️encode/🦀️.rs`; each `new(maximum_bytes: usize, callback: &mut dyn FnMut(Native{Decode,Encode}Progress)->bool)` has no SqliteDatabaseLimits argument. Each exposes maximum_bytes()/owned_bytes(), checkpoint(), begin_stage(total), step()/advance(units), scoped_depth(maximum,closure), scoped_stage(closure), scoped_maximum(maximum,closure). Actual decode charge admits cumulative storage before copy/reserve; caller maximum is a byte allowance, not max_value_bytes or a progress total. Encode admit_capacity(self,source_bytes,multiples,scaffold_bytes) consumes/returns the same admission owner and should not be substituted for supplied caller limits in Chart demand.

SqliteDatabaseLimits has max_file_bytes,max_value_bytes,max_allocation_bytes,max_schema_bytes,max_rows,max_columns,max_tables,max_pages. SqliteSnapshotControl::new(callback,limits), limits() and allocation_stage at139/149 retain actual multi-stage allowance. Exact allocation-stage signature is a closure receiving `(remaining: usize, checkpoint: &mut dyn FnMut(usize,usize)->bool)` and returning `(Result<T,E>,actual_owned_bytes)`; outer Result<ValueError> plus inner owner result remains distinct. Provider demand should route native controls through that same retained stage, and assert interior cancellation after actual admitted copy work rather than canceling only borrowed UTF8 validation.
