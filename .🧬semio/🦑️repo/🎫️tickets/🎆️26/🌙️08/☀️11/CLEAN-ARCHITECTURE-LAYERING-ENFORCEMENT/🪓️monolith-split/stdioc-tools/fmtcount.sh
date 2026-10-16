#!/bin/zsh
# 🎨️ Counts rustfmt diff hunks over every Rust file of a tree (skip_children, repo config); equal-or-lower than the baseline proves the carve added no formatting debt.
tree="$1"
total=0
for f in $(find "$tree" -name '*.rs' -not -path '*/node_modules/*' -not -path '*/dist/*' | sort); do
  n=$(rustfmt +nightly-2026-07-20 --edition 2021 --check --config skip_children=true,unstable_features=true,max_width=250,use_small_heuristics=Max,merge_derives=true,use_field_init_shorthand=true,use_try_shorthand=true,remove_nested_parens=true "$f" 2>&1 | /usr/bin/grep -c '^Diff in')
  total=$((total + n))
  [ "$n" != "0" ] && echo "$n $f" | cut -c1-200
done
echo "TOTAL $total"
