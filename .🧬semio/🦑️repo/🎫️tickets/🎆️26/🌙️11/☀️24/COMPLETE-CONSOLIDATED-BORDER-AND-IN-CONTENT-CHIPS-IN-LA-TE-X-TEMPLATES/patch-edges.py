from semio_patch import patch

L = "🧰️framework/🛍️products/📓️print/🖋️latex/"
patch(L + "semio-core.sty", [
    (r"""\newcommand{\semio@list@divider}{%
  \par
  \begingroup""", r"""\newif\ifsemio@list@closing
\newcommand{\semio@list@divider}{%
  \par
  \global\semio@list@closingfalse
  \begingroup"""),
    (r"""\newcommand{\semio@list@close}{\semio@list@divider\let\item\semio@list@item\xdef\semio@list@tail{\the\dimexpr\@topsepadd\relax}}""",
     r"""% 🪟️ Inside a window the closing line is owed to whatever follows the list: a
% paragraph draws it, the window's own bottom border replaces it.
\newcommand{\semio@list@close}{%
  \ifsemio@column@framed
    \par\global\semio@list@closingtrue
  \else
    \semio@list@divider
  \fi
  \let\item\semio@list@item
  \xdef\semio@list@tail{\the\dimexpr\@topsepadd\relax}%
}
\newcommand{\semio@list@closing@draw}{%
  \ifsemio@list@closing
    \global\semio@list@closingfalse
    \ifsemio@column@framed
      \begingroup
      \dimen@\prevdepth
      \nointerlineskip
      \moveleft\semio@window@bodypad\vbox to\z@{\vss\hbox{\textcolor{semio-chrome-border-normal}{\vrule width\dimexpr\hsize+2\semio@window@bodypad\relax height\semio@stroke@hairline}}}%
      \nointerlineskip
      \prevdepth\dimen@
      \endgroup
    \fi
  \fi
}
\AddToHook{para/before}{\semio@list@closing@draw}
% 📐️ A list of the column keeps the prose inset: its text block moves in by the
% body padding and narrows by as much, so labels never touch the column border.
\newcommand{\semio@list@inset}{%
  \ifsemio@column@framed\else\ifnum\@listdepth=\@ne
    \advance\@totalleftmargin\semio@window@bodypad\relax
    \advance\linewidth-\semio@window@bodypad\relax
    \parshape\@ne\@totalleftmargin\linewidth
  \fi\fi
}
\AddToHook{cmd/list/after}{\semio@list@inset}
% 🧮️ A display of the column is set inside the prose inset, so its number stays clear of the border.
\newcommand{\semio@display@inset}{%
  \ifsemio@column@framed\else
    \advance\displayindent\semio@window@bodypad\relax
    \advance\displaywidth-2\dimexpr\semio@window@bodypad\relax
  \fi
}
\AtBeginDocument{\everydisplay\expandafter{\the\everydisplay\semio@display@inset}}"""),
])
patch(L + "semio-window.sty", [
    (r"""    before~upper={\semio@column@bleed},""", r"""    before~upper={\semio@column@bleed},
    after~upper={\global\semio@list@closingfalse},"""),
])
print("ok")
