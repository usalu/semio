#!/bin/bash
p="$1"
rg -n --glob '!node_modules' --glob '!*🎫️tickets*' --glob '!.cache' --glob '!dist' --glob '!bazel-*' --glob '!temp' -g '*.ts' -g '*.tsx' -g '*.mjs' -g '*.js' -g '*.mts' "(from|import|require)\s*\(?\s*['\"]${p}(/[^'\"]*)?['\"]" /Users/ueli/Documents/semio 2>/dev/null | cut -c1-260
