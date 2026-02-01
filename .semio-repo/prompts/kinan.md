

# Prompt history

Slection implemetation Kit Editor  :

 5 Prompt plan based on design-to-kit-selection-plan
[Provide Design.tsx + Kit.tsx files]

**Prompt A**: In these files, identify all selection entry points...
[Wait for response, review output]

**Prompt B**: Now in Kit.tsx, list the existing selection surface...
[Wait for response, review output]

**Prompt C**: Based on Prompts A & B, propose generic helpers...
[Review output. If it's good, proceed. If not, refine with: "Make it simpler by..."]

**Prompt D**: Implement these helpers into Kit.tsx...
[Get code, review it, ask for tweaks]

**Prompt E**: Create tests for the helpers...
[Get test outline]






Kit Editor plan:

Features : 



allow multiple filters at once

Change D3 Force parameters 

introduce multiple select of multiple nodes through pressing shift or rectangular selection




Bugs:

Diagram automatically selects multiple nodes as if i was clicked on shift

The tables left and right arent aligned (Hierachie table doesnt match with Diagram window)


Nodes and edges arent aligned 






There is a mismatch between the avatar circle and the edges. The edges are offset on a larger circle making it appear as the node is larger.

Implement the D3 simulation to match the example.
Currently the rest of the nodes dont move while dragging a single node.
There is no simulation. There seems to be a fundamental state issue.
Analyze in depth what could be the core problem and fix everything.

The red selection should only heighligh the node outline like in the design app 

i want to fix the diagram 

previously : adapt the atlas by explaining all general concepts with concrete examples, workflow, programming langaues, package manager, programming styles, repo structure, use cases etc exclusively from semio. do it one section at a time
Now: I like the resault but combine.old with the new where you explain general concepts first with examples from semios programing languages, workflows, architecture, package manager etc.. Simplify the lanague for non dev to start learning while keeping all techical terms and comlexity. always comment code and its systemetic thinking by relating it to the main concept. this should be an intro to programming speciffically on semio as an example guide

Extract prompt strategies out to plans/prompt-strategy.md 