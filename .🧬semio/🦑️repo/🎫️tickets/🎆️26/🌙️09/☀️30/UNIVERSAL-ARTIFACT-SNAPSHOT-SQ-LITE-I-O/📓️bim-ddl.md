# Current BIM Relational Schema

The literal schema owns all current persisted BIM entity families. Each parameter branch has named columns with closed presence constraints. Owned ordered children and empty collection parents have explicit relational owners. References to authored domain keys remain literal TEXT, including unresolved external references. Floating values have query REAL, signed binary64 word and classification columns. No BLOB or hidden JSON content is present.

The independent Bun SQLite engine created 93 tables; the largest table has 59 columns. This schema admission is not a provider round-trip qualification. Source and Native implementations must still prove the complete current witness over original public IO.
