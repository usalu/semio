# Generalize Commit Metrics

## Target footer layout

```
📊️metric📃uloc💯️…
📊️metric💾size💯️…
📊️metric🦀️rust📃uloc💯️…
📊️metric🦀️rust💾size💯️…
```

## Size vs uloc file sets

- **uloc**: text files ≤8MB, same skip rules as before.
- **size**: same path/language filters but `stat.size` on all matched files (binaries and large files included).

## Cache

- `.git/compose-metrics-cache.json` version 5 (uloc + size maps).
- Old `compose-uloc-cache.json` is not read.

## Bundle inline metrics

Bundle scope and day lines append both kinds: `📊️metric📃uloc…📊️metric💾size…`.
