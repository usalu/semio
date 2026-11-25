#!/bin/zsh
# The bare cargo for the one build that cannot go through `repo cargo`; the jobs it may use are the grant it runs under (`repo capacity run` hands them over as SEMIO_BUILD_PARALLEL).
export CARGO_BUILD_JOBS=${SEMIO_BUILD_PARALLEL:-3}
exec $HOME/.cargo/bin/cargo "$@"
