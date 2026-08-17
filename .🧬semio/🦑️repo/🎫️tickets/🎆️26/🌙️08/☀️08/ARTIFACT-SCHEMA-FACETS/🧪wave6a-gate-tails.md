### cargo check -p semio-framework-os-kernel
```

warning: `semio-framework-os-kernel` (lib) generated 45 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 22 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.20s
```

### cargo check -p semio-framework-plugin
```

warning: `semio-framework-plugin` (lib) generated 15 warnings (run `cargo fix --lib -p semio-framework-plugin` to apply 15 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 3.13s
```

### cargo check -p semio-framework-os --features os-host-full
```

warning: `semio-framework-os` (lib) generated 36 warnings (run `cargo fix --lib -p semio-framework-os` to apply 4 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 4.87s
```

### cargo test -p semio-s-plugin-puzzle --lib
```

test result: ok. 390 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.47s

```

### bun ./📜️script.ts policy 2>&1 | rg -i puzzle
```
(empty — no lines matched)
```
