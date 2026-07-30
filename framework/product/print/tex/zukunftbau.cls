\NeedsTeXFormat{LaTeX2e}
\ProvidesExplClass{zukunftbau}{2026/07/06}{0.1.0}{Zukunft Bau report family}

\RequirePackage{kvoptions}

\SetupKeyvalOptions{
  family=zukunftbau,
  prefix=zukunftbau@,
  setkeys=\kvsetkeys
}

\DeclareStringOption[forschungsbericht]{type}
\DeclareStringOption[light]{theme}
\ProcessKeyvalOptions*

\ExplSyntaxOn
\str_case:VnF \zukunftbau@type {
  { forschungsbericht } {
    \LoadClass[language=de,theme=\zukunftbau@theme,type=report,citestyle=numeric]{semio}
  }
  { zwischenbericht } {
    \LoadClass[language=de,theme=\zukunftbau@theme,type=paper,citestyle=numeric]{semio}
    \SemioSetup{appendix-level=part}
  }
  { kompaktbericht } {
    \LoadClass[language=de,theme=\zukunftbau@theme,type=paper,citestyle=authoryear]{semio}
    \SemioSetup{kompakt-draft=true}
  }
} {
  \LoadClass[language=de,theme=\zukunftbau@theme,type=report]{semio}
}
\ExplSyntaxOff
