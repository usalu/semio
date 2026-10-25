#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test -p os-hub -p semio-hub 
