#!/bin/zsh
export LC_ALL=en_US.UTF-8
cd /Users/ueli/Documents/semio
S=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-1dOdZf/35823786-c219-418d-81dc-737e033e7539/scratchpad/F67
out=$(repo typecheck "$@" 2>&1)
echo "$out" | grep -E "error TS" > $S/last-errors.txt
grep -v "🧪️tests" $S/last-errors.txt > $S/last-product.txt
echo "product errors: $(wc -l < $S/last-product.txt)  test errors: $(grep -c '🧪️tests' $S/last-errors.txt)"
head -20 $S/last-product.txt | cut -c1-400
echo "$out" | grep -E "^typecheck: (passed|FAILED)|drift"
