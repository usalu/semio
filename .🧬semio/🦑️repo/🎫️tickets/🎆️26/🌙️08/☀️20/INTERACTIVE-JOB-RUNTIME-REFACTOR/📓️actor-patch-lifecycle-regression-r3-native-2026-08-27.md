# Actor Lifecycle Regression After Patch Receipt — Native R3

Canonical Actor selector `actor_instance_lifecycle_` executed after the new patch-receipt codec: 5 passed, 0 failed, 100 skipped; nextest 0.211s, exit 0.

```text
Summary [0.211s] 5 tests run: 5 passed, 100 skipped
NX Successfully ran target test for project @semio-tech/framework-actor-rs
```

Together with the separately executed new patch-receipt three-law R2, these are eight passing scoped Actor laws. They are not the full Actor suite or native guest lifecycle integration. The lifecycle owner may now join the Kernel/WIT consumers coherently.

Raw output: `🧪️member-actor-patch-lifecycle-regression-r3-native-2026-08-27.txt`.
