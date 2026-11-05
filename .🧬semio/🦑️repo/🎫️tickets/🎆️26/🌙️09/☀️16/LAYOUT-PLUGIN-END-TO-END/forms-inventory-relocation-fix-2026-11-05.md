# Forms Inventory Relocation

Corrected exactly five declared Cargo-manifest user coordinates in the dependency inventory after the Forms config source moved. The current source manifest exists; its former editor-child path does not. Package versions, dependency metadata and baseline metadata are unchanged. This repairs the graph admission failure observed by the native dependency owner; no graph-inference rule was relaxed.
