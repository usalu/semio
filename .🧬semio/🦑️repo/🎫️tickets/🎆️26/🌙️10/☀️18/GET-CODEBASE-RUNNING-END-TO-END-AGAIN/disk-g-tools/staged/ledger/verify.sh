#!/bin/sh
# laws of the batch ledger on a fresh mirror: sh verify.sh [before] (before: the new laws against the unedited tree, they must fail)
exec sh "$(dirname "$0")/../../verify.sh" ledger "$@"
