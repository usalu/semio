#!/bin/sh
compiler=$1
shift
case " $* " in
  *" --crate-name semio_s_artifact_stdio_pdf_model "*)
    exec "$compiler" "$@" -Z threads=1
    ;;
  *)
    exec "$compiler" "$@"
    ;;
esac
