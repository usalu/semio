#!/bin/sh
# Measures peak RSS and wall time per rustc unit; appends "crate<TAB>rssBytes<TAB>wallSeconds<TAB>kind" to $SEMIO_MEM_TSV.
rustc=$1; shift
name=""; prev=""; kind=lib
for a in "$@"; do
  [ "$prev" = "--crate-name" ] && name=$a
  [ "$a" = "--test" ] && kind=test
  prev=$a
done
[ -z "$name" ] && exec "$rustc" "$@"
tmp=$(mktemp "${TMPDIR:-/tmp}/memwrap.XXXXXX")
start=$(date +%s)
case "$(uname -s)" in
  Darwin) /usr/bin/time -l -o "$tmp" "$rustc" "$@"; code=$?; rss=$(awk '/maximum resident set size/{print $1}' "$tmp");;
  *) /usr/bin/time -v -o "$tmp" "$rustc" "$@"; code=$?; rss=$(awk -F: '/Maximum resident set size/{gsub(/ /,"",$2);print $2*1024}' "$tmp");;
esac
end=$(date +%s)
printf '%s\t%s\t%s\t%s\n' "$name" "${rss:-0}" "$((end-start))" "$kind" >> "${SEMIO_MEM_TSV:?}"
rm -f "$tmp"
exit $code
