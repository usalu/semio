\NeedsTeXFormat{LaTeX2e}
\ProvidesExplClass{semio}{2026/07/06}{0.1.0}{semio printable documents}

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
\RequirePackage{semio-components}
\RequirePackage{semio-window}

\edef\semio@setup@type{\semio@type}
\edef\semio@setup@theme{\semio@theme}
\edef\semio@setup@language{\semio@language}
\ExplSyntaxOn
\keys_set:nn { semio / setup } {
  type = \semio@setup@type,
  theme = \semio@setup@theme,
  language = \semio@setup@language,
}
\ExplSyntaxOff

\RequirePackage[ngerman,english]{babel}
\RequirePackage{microtype}
\RequirePackage{hyperref}
