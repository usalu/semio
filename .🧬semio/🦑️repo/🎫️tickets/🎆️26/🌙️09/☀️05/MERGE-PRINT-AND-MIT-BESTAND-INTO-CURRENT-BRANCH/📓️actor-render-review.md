# Actor Network Rendering Review

Imported ledger validates 798 nodes, 444 edges and 14 programs. Re-rendered the three deterministic figures/tables/program fragments after adapting source paths. Edge fingerprint remained `ae684352b0c7` (prefix). The incoming stored figures were stale for the current renderer. First normalized differences:


```diff
temp/merge/mit-bestand/bericht/forschungsbericht/anhang/akteursnetz-figuren.tex
--- 
+++ 
@@ -691,133 +691,133 @@
 \end{GraphFigure}
 
 \begin{GraphFigure}[title={Deutschland \textperiodcentered\ 66 Organisationen \textperiodcentered\ 23 Projekte}, width=181.00, height=90.00]
-\SemioGraphEdge{139.66,87.25}{153.13,79.62}
-\SemioGraphEdge{3.43,13.64}{7.88,28.00}
-\SemioGraphEdge{27.00,46.25}{15.11,45.62}
-\SemioGraphEdge{15.11,45.62}{9.08,59.86}
-\SemioGraphEdge{15.11,45.62}{7.88,28.00}
-\SemioGraphEdge{168.31,87.01}{178.31,87.31}
-\SemioGraphEdge{178.01,33.92}{168.19,43.73}
-\SemioGraphEdge{178.01,33.92}{178.25,21.24}
-\SemioGraphEdge{178.01,33.92}{178.25,47.15}
-\SemioGraphEdge{60.24,44.72}{73.85,55.25}
-\SemioGraphEdge{60.24,44.72}{73.81,37.26}
-\SemioGraphEdge{60.24,44.72}{69.27,45.93}
-\SemioGraphEdge{109.96,36.59}{113.08,51.72}
-\SemioGraphEdge{153.13,79.62}{151.23,64.99}
-\SemioGraphEdge{153.13,79.62}{152.73,87.31}
-\SemioGraphEdge{153.13,79.62}{140.89,66.16}
-\SemioGraphEdge{150.76,49.88}{140.89,66.16}
-\SemioGraphEdge{46.59,20.79}{55.34,11.98}
-\SemioGraphEdge[kind=muted]{119.80,79.28}{119.78,87.25}
-\SemioGraphEdge{140.89,66.16}{135.61,49.36}
-\SemioGraphEdge{140.89,66.16}{133.26,80.92}
-\SemioGraphEdge[kind=muted]{86.22,69.96}{92.14,45.25}
-\SemioGraphEdge[kind=muted]{86.15,26.65}{92.14,45.25}
-\SemioGraphEdge{84.76,48.72}{101.72,47.25}
-\SemioGraphEdge{120.30,44.99}{101.72,47.25}
-\SemioGraphEdge{135.26,10.66}{127.84,5.92}
-\SemioGraphEdge{37.33,53.53}{49.42,61.49}
-\SemioGraphEdge{68.29,72.91}{49.42,61.49}
-\SemioGraphEdge{77.86,13.65}{65.28,7.40}
-\SemioGraphEdge{135.26,10.66}{140.04,26.20}
-\SemioGraphEdge{153.30,37.64}{140.04,26.20}
-\SemioGraphEdge{126.79,41.07}{140.04,26.20}
-\SemioGraphEdge{136.51,2.70}{120.73,3.00}
-\SemioGraphEdge{110.34,14.42}{120.73,3.00}
-\SemioGraphEdge{178.31,81.92}{178.01,71.78}
-\SemioGraphEdge{46.59,20.79}{39.85,32.47}
-\SemioGraphEdge{24.11,30.95}{39.85,32.47}
-\SemioGraphEdge{51.81,39.17}{39.85,32.47}
-\SemioGraphNode[state=hypo,image={asset/akteur/DE/I01.png}]{139.66,87.25}{I01}
-\SemioGraphNode[state=attested,image={asset/akteur/DE/U01.png}]{135.61,49.36}{U02}
-\SemioGraphNode[image={asset/akteur/DE/F01.png}]{135.26,10.66}{F01}
-\SemioGraphNode[state=focal]{153.13,79.62}{P3}
-\SemioGraphNode[state=attested,image={asset/akteur/DE/N01.png}]{55.34,11.98}{N01}
-\SemioGraphNode[image={asset/akteur/DE/M01.png}]{105.41,65.25}{M01}
+\SemioGraphEdge{140.03,87.25}{153.57,79.59}
+\SemioGraphEdge{3.51
```

## Shared Asset Path Reconciliation

Concurrent semantic asset renames were retained. Regenerated fragments from the current ledger before final build validation. First differences:

```diff

```
