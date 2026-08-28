# Common Kernel Return Source Layout R1

Canonical `@semio-tech/framework-rs:test-wire-retirement-native --args='--lib return_source_native_layout_census_ -- --nocapture'` completed exit 0 in the existing shared native target. Raw: `🧪️member-kernel-return-source-layout-r1-native-2026-08-27.txt`.

```text
[DEBUG] return-source native layout census {"borrowedMessageCursorBytes":96,"effectAlignment":16,"effectBytes":192,"effectPageDescriptorBytes":24,"fixedReturnPageBytes":4098,"fixedReturnResultBytes":4144,"nativeOwnerMounted":false,"pointerBytes":8,"presenceAlignment":8,"presenceBytes":576,"presencePageDescriptorBytes":24,"sourceBackingAdmitted":false,"turnResultAlignment":8,"turnResultBytes":2040,"uiTurnPatchBytes":1768}
Summary [0.016s] 1 test run: 1 passed, 260 skipped
```

This is an actual native size/alignment inventory only. It explicitly proves neither admitted source backing nor a mounted native return owner. Dag owns the source design and follow-up implementation.
