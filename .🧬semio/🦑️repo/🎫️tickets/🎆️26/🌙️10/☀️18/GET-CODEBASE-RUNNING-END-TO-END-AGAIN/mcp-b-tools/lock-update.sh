#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
exec nice -n 10 repo cargo --store=shared --configuration=prototype -- update --workspace --offline
