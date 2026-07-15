---
name: Fix Muted Chip Fill Overshoot
overview: The muted (non-colored) window chips render their canvas-colored fill wider and taller than their border because the fill uses the default `\fboxsep` padding instead of `0pt`, causing overshoot to the right and bottom. The colored chips already reset `\fboxsep` to `0pt` before painting, which is why they look correct.
todos:
 - id: fix-fboxsep
   content: Add \setlength{\fboxsep}{0pt} guard around \colorbox in \semio@heading@cap@muted@core in print/tex/semio-window.sty
   status: completed
 - id: rebuild
   content: Rebuild zwischenbericht via bun nx run @semio-tech/mit-bestand-bericht:build
   status: completed
 - id: verify
   content: Re-render and visually verify paragraph chips and window header-row chips no longer overshoot
   status: completed
 - id: update-ticket
   content: Update verify-log.md and ticket with fix summary and new screenshots
   status: completed
isProject: false
---

## Root cause

In [print/tex/semio-window.sty](print/tex/semio-window.sty), compare the two chip-painting macros:

- `\semio@window@cap` (colored chips, e.g. `Arbeitspakete`, `AP-Erfahrung`) — correctly resets `\fboxsep` to `0pt` right before the `\colorbox` call:

```977:989:print/tex/semio-window.sty
        \begingroup
        \setlength{\fboxsep}{0pt}%
        \colorbox{#1}{%
          \vbox to \semio@window@cap@body@h{%
            \vfil
            \nointerlineskip
            \hbox to \semio@window@cap@w{%
              \hfil\usebox{\semio@window@cap@slot}\hfil
            }%
            \vfil
          }%
        }%
        \endgroup
```

- `\semio@heading@cap@muted@core` (muted chips, e.g. `Recherche`, `Bauteilbörsen`, and the title/number chips in the first header row of `Blockquote`/`Image`/etc. windows) — is **missing** that reset, so `\colorbox` pads the content with the document's default `\fboxsep` (commonly `3pt`) on all sides:

```1030:1041:print/tex/semio-window.sty
    \hbox to \semio@window@cap@w{%
      \colorbox{semio-chrome-canvas}{%
        \vbox to \semio@window@cap@body@h{%
          \vfil
          \nointerlineskip
          \hbox to \semio@window@cap@w{%
            \hfil\usebox{\semio@window@cap@slot}\hfil
          }%
          \vfil
        }%
      }%
    }%
```

Because the colorbox becomes `\semio@window@cap@w + 2\fboxsep` wide and `\semio@window@cap@body@h + 2\fboxsep` tall, but it's placed inside a rigid `\hbox to \semio@window@cap@w` / `\vbox to \semio@chrome@titlebar@height`, the extra padding cannot shrink away — it overflows past the target box on the right and bottom. The border strokes (`\semio@window@stroke@h`/`\semio@window@stroke@v`, drawn as `\rule`s) are unaffected since they don't go through `\colorbox`, which is why the border looks correct while the fill overshoots.

This single macro (`\semio@heading@cap@muted@core`) is shared by:

- `\semio@heading@cap@muted` and `\semio@heading@cap@muted@tab` — used for muted paragraph chips (`Recherche`, `Bauteilbörsen`) and muted number/title chips in the header row (first line) of kind windows (`Blockquote`, `Image`, etc.) via `\semio@window@header@muted`.

So fixing it in one place fixes both reported symptoms.

## Fix

In `\semio@heading@cap@muted@core` in [print/tex/semio-window.sty](print/tex/semio-window.sty), wrap the `\colorbox{semio-chrome-canvas}{...}` call with the same `\begingroup \setlength{\fboxsep}{0pt} ... \endgroup` guard already used in `\semio@window@cap`:

```latex
\newcommand{\semio@heading@cap@muted@core}[1]{%
  \semio@window@cap@body@h@set
  \sbox{\semio@window@cap@slot}{%
    \begingroup
    \semio@heading@chip@font\color{semio-chrome-text-normal}%
    \hspace{\semio@chrome@padding}#1\hspace{\semio@chrome@padding}%
    \endgroup
  }%
  \setlength{\semio@window@cap@w}{\wd\semio@window@cap@slot}%
  \vbox to \semio@chrome@titlebar@height{%
    \nointerlineskip
    \hbox to \semio@window@cap@w{%
      \semio@window@stroke@h{semio-chrome-border-normal}{\semio@window@cap@w}%
    }%
    \nointerlineskip
    \hbox to \semio@window@cap@w{%
      \begingroup
      \setlength{\fboxsep}{0pt}%
      \colorbox{semio-chrome-canvas}{%
        \vbox to \semio@window@cap@body@h{%
          \vfil
          \nointerlineskip
          \hbox to \semio@window@cap@w{%
            \hfil\usebox{\semio@window@cap@slot}\hfil
          }%
          \vfil
        }%
      }%
      \endgroup
    }%
  }%
}
```

## Verification

1. Kill any stray `tectonic` processes, then rebuild: `bun nx run @semio-tech/mit-bestand-bericht:build` from the repo root.
2. Re-render the affected pages (paragraph chips `Recherche`/`Bauteilbörsen`, and a kind window header row e.g. `Blockquote`) to a high-zoom PNG and visually confirm the canvas fill no longer overshoots the border on the right/bottom edges, matching the alignment already seen on colored chips (`Arbeitspakete`, `AP-Erfahrung`).
3. Update `.repo/🎫/26/07/09/FIX-PRINT-PARAGRAPH-CHIP-ALIGNMENT/verify-log.md` with the fix and new verification screenshots, then close/reopen the ticket per repo workflow as needed.
