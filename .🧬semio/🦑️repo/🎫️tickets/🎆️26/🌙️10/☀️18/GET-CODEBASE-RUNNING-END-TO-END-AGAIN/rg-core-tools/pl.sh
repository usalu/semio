#!/bin/zsh
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":7500000}' perl -e 'alarm($ENV{ALARM}||590); exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- "$@"
