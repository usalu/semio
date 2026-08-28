# Exact Comparison Frame Storage — R66–R68

The language-neutral comparison fixture retains 256 logical frames and 256 page slots. Each frame encodes two checked 16-bit page indices, a checked 16-bit text operand position, one remembered byte and one phase flag. The sentinel is 65535; the maximum live page is 255 and maximum two-operand text position is 1024. Node Buffer independently produces the fixture's eight-byte projection; serde and the existing 18-component/seven-hostile native comparison laws remain the semantic oracle.

R66 stopped before tests because the new sibling test could not name private `ValueFrame`. Only test visibility/import was corrected. R67 then executed the intended semantic RED: **0 passed, 1 failed, 155 skipped**, 0.019s; actual old frame storage was 32 bytes rather than 8.

After checked domain conversion, R68 executed `retained_component_compare_`: **3 passed, 153 skipped**, 0.039s; process 54019 exited 0. Actual output:

```text
[DEBUG] comparison-frame bytes=8 depth=256 cursor=2208
[DEBUG] retained-component-compare variants=18 hostile-values=7 byte-grants=1,64,4096 exact-serde=true
Summary [0.039s] 3 tests run: 3 passed, 153 skipped
```

No Box or heap allocation was introduced. The fixed cursor's actual 2208-byte initialization fits the unchanged 4096-byte component work grant. Runtime root binding and near-grant completion transfers still require their own tests; these three tests do not certify the live reconciler, complete resident census or Process workshop.

Canonical route: `@semio-tech/ui-contract-rs:test --args='--lib retained_component_compare_ -- --nocapture'`, unchanged master Cargo target/environment. Raw logs: `🧪️member-ui-compare-frame-red-r66-native-2026-08-27.txt`, `🧪️member-ui-compare-frame-red-r67-native-2026-08-27.txt`, `🧪️member-ui-compare-frame-green-r68-native-2026-08-27.txt`.
