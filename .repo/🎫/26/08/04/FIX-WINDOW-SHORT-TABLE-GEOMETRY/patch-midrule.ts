#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";

const path = "print/tex/semio-table.sty";
let t = readFileSync(path, "utf8");

const start = t.indexOf("\\newcommand{\\semio@table@rule}{%");
const end = t.indexOf("\\newcommand{\\semio@table@row@sep}{%", start);
if (start < 0 || end < 0) {
  console.error("[DEBUG] markers not found", start, end);
  process.exit(1);
}

const neu = `\\newcommand{\\semio@table@rule}{%
  \\noalign{%
    \\begingroup
      \\color{semio-chrome-border-normal}%
      \\ifsemio@table@owns@sides
        \\hbox to \\linewidth{%
          \\kern\\arrayrulewidth
          \\vrule height\\arrayrulewidth depth\\z@
                 width\\dimexpr\\linewidth-2\\arrayrulewidth\\relax
          \\kern\\arrayrulewidth
        }%
        \\vskip-\\arrayrulewidth
        \\hbox to \\linewidth{%
          \\smash{%
            \\raisebox{-3pt}{%
              \\vrule width\\arrayrulewidth
                     height\\dimexpr\\arrayrulewidth+6pt\\relax depth\\z@
            }%
          }%
          \\hss
          \\smash{%
            \\raisebox{-3pt}{%
              \\vrule width\\arrayrulewidth
                     height\\dimexpr\\arrayrulewidth+6pt\\relax depth\\z@
            }%
          }%
        }%
        \\vskip\\arrayrulewidth
      \\else
        \\hrule height\\arrayrulewidth width\\linewidth
      \\fi
    \\endgroup
  }%
}
`;

t = t.slice(0, start) + neu + t.slice(end);

const commentStart = t.indexOf("% owns@sides: full-width");
const commentEnd = t.indexOf("\\newcommand{\\semio@table@rule}{%", commentStart);
if (commentStart >= 0 && commentEnd > commentStart) {
  const comment = `% owns@sides: mid-rule paints ONLY the inner span (between the L/R border
% slots) so the outer edge is never crossed by horizontal ink. Smashed L/R
% pillars fill the join band and overlap 3pt into the adjacent rows, welding
% the per-row side strokes into one continuous outer edge. Smash keeps band
% advance = one hairline.
%
% Windowed short tables: the tcolorbox leftrule/rightrule (plus TikZ side
% overstroke) is the sole continuous outer edge. Mid-rules stay exactly
% \\linewidth so they meet the INNER face of that frame.
`;
  t = t.slice(0, commentStart) + comment + t.slice(commentEnd);
}

writeFileSync(path, t);
console.log("[DEBUG] patched", t.includes("raisebox{-3pt}"), t.includes("\\kern\\arrayrulewidth"));
