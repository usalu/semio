\NeedsTeXFormat{LaTeX2e}
\ProvidesExplClass{semio}{2026/07/09}{0.2.0}{semio printable documents}

\RequirePackage{expl3}
\RequirePackage{xparse}
\RequirePackage{kvoptions}

\SetupKeyvalOptions{
  family=semio,
  prefix=semio@,
  setkeys=\kvsetkeys
}

\DeclareStringOption[report]{type}
\DeclareStringOption[light]{theme}
\DeclareStringOption[de]{language}
\DeclareStringOption[numeric]{citestyle}
\ProcessKeyvalOptions*

\ExplSyntaxOn
\str_case:VnF \semio@type {
  { flyer } {
    \providecommand { \semio@page@margin } { 1.2cm }
    \RequirePackage[paper=a4paper,landscape,margin=1.2cm]{geometry}
  }
  { paper } {
    \providecommand { \semio@page@margin } { 2.5cm }
    \RequirePackage[paper=a4paper,margin=2.5cm,twoside,bindingoffset=8mm]{geometry}
  }
  { report } {
    \providecommand { \semio@page@margin } { 2.5cm }
    \RequirePackage[paper=a4paper,margin=2.5cm,twoside,bindingoffset=8mm]{geometry}
  }
} {
  \providecommand { \semio@page@margin } { 2.5cm }
  \RequirePackage[paper=a4paper,margin=2.5cm,twoside,bindingoffset=8mm]{geometry}
}
\str_case:VnF \semio@type {
  { report } { \LoadClass[twoside=true,open=right,cleardoublepage=empty]{scrreprt} }
  { paper } { \LoadClass[twoside=true]{scrartcl} }
  { flyer } { \LoadClass{scrartcl} }
} {
  \LoadClass[twoside=true,open=right,cleardoublepage=empty]{scrreprt}
}
\ExplSyntaxOff

\RequirePackage{fontspec}
\RequirePackage{semio-tokens}
\RequirePackage{semio-fonts}
\RequirePackage{semio-core}
\RequirePackage{semio-logo}
\RequirePackage{semio-table}

\ExplSyntaxOn
\keys_set:nx { semio / setup } {
  type = \semio@type,
  theme = \semio@theme,
  language = \semio@language,
}
\str_case:VnF \semio@language {
  { de } { \PassOptionsToPackage { main = ngerman } { babel } }
} { }
\ExplSyntaxOff

\RequirePackage[ngerman,english]{babel}
\RequirePackage{microtype}
\RequirePackage[style=\semio@citestyle,backend=bibtex,sorting=nyt]{biblatex}
\RequirePackage{semio-window}
\RequirePackage{semio-components}
\RequirePackage{hyperref}
\hypersetup{colorlinks,linkcolor=semio-primary,citecolor=semio-primary,urlcolor=semio-primary,filecolor=semio-primary}
