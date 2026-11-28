#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
export NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_LANGUAGE=en
exec nice -n 0 repo run @semio-tech/semio-tech-play:serve-forms-e2e
