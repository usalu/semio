# Canonical Document Assembly Accounting — Native GREEN R52

Canonical UI selector `retained_document_assembly_`: actual 4 passed, 0 failed, 143 skipped; nextest 0.236s, exit 0. The new metadata law first failed at R51, then passed after separating initialized page metadata from uninitialized record capacity.

```text
[DEBUG] document-metadata allocated=7568 initialized=1152 expected-initialized=1152
Summary [0.236s] 4 tests run: 4 passed, 143 skipped
```

Physical capacity and initialized work are distinct. Allocation errors now preserve both actual retained allocation and initialized metadata counts. Fixed ID comparisons report operand bytes separately. All exact input/root/read ownership laws from R50 remain in this run.

Raw output: `🧪️member-ui-document-assembly-green-r52-native-2026-08-27.txt`. Active runtime old-record R30/R31 remain RED pending canonical-root adoption; this result is not Process resident acceptance or a wall-clock proof.
