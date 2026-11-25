#!/usr/bin/env bash
# Samples the peak RSS (KB) of the rustc compiling semio_s_artifact_stdio_pdf_model; exits when p3-pdfmodel.log has RC:
G="$(cd "$(dirname "$0")" && pwd)/🗑️generated"; max=0
while ! grep -q '^RC:' "$G/p3-pdfmodel.log" 2>/dev/null; do
  for p in $(pgrep -f 'crate-name semio_s_artifact_stdio_pdf_model'); do
    r=$(awk '/VmHWM/{print $2}' /proc/$p/status 2>/dev/null); [ -n "$r" ] && [ "$r" -gt "$max" ] && { max=$r; echo "$(date +%T) pid=$p peakRSS_KB=$max" >> "$G/p3-pdfmodel-rss.log"; }
  done; sleep 2
done
echo "$(date +%T) final peakRSS_KB=$max" >> "$G/p3-pdfmodel-rss.log"
