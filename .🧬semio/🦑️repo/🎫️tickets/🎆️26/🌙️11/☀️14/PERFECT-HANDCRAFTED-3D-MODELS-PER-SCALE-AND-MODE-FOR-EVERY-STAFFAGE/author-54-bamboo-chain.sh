#!/bin/sh
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
G="🗑️generated/author-54"
$BUN author-54-lobes.ts vegetation/bamboo-clump 100 --n 10 --rounds 5000 --cell 0.05 --material bamboo-leaf --ids foliage --fixed $G/bamboo.s100.culms.json --out $G/bamboo.s100.lobes.json --reset --bbox > $G/bamboo100.fit.txt 2>&1
$BUN author-54-lobes.ts vegetation/bamboo-clump 200 --n 8 --rounds 5000 --cell 0.06 --material bamboo-leaf --ids foliage --fixed $G/bamboo.s200.culms.json --out $G/bamboo.s200.lobes.json --reset --bbox > $G/bamboo200.fit.txt 2>&1
