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
  { flyer } { \RequirePackage[paper=a4paper,landscape,margin=1.2cm]{geometry} }
  { paper } { \RequirePackage[paper=a4paper,margin=2.5cm]{geometry} }
  { report } { \RequirePackage[paper=a4paper,margin=2.5cm]{geometry} }
} {
  \RequirePackage[paper=a4paper,margin=2.5cm]{geometry}
}
\str_case:VnF \semio@type {
  { report } { \LoadClass{scrreprt} }
  { paper } { \LoadClass{scrartcl} }
  { flyer } { \LoadClass{scrartcl} }
} {
  \LoadClass{scrreprt}
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
\str_case:Vn \semio@language {
  { de } { \PassOptionsToPackage { main = ngerman } { babel } }
} { }
\ExplSyntaxOff

\RequirePackage[ngerman,english]{babel}
\RequirePackage{microtype}
\RequirePackage[style=\semio@citestyle,backend=bibtex,sorting=nyt]{biblatex}
\RequirePackage{semio-window}
\RequirePackage{semio-components}
\RequirePackage{hyperref}
