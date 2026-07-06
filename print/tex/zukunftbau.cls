\NeedsTeXFormat{LaTeX2e}
\ProvidesExplClass{zukunftbau}{2026/07/06}{0.1.0}{Zukunft Bau report family}

\RequirePackage{kvoptions}

\SetupKeyvalOptions{
  family=zukunftbau,
  prefix=zukunftbau@,
  setkeys=\kvsetkeys
}

\DeclareStringOption[forschungsbericht]{type}
\ProcessKeyvalOptions*

\ExplSyntaxOn
\str_case:VnF \zukunftbau@type {
  { forschungsbericht } {
    \LoadClass[language=de,theme=light,type=report]{semio}
  }
  { zwischenbericht } {
    \LoadClass[language=de,theme=light,type=paper]{semio}
  }
  { kompaktbericht } {
    \LoadClass[language=de,theme=light,type=paper]{semio}
    \SemioSetup{kompakt-draft=true}
    \RequirePackage[style=authoryear,backend=bibtex]{biblatex}
    \addbibresource{references.bib}
  }
} {
  \LoadClass[language=de,theme=light,type=report]{semio}
}
\ExplSyntaxOff
