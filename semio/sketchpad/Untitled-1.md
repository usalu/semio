---

semio: Extend designs with copy and paste functionality.

---

Definitions:

A selection is a set of pieces and connections.

A piece is:

- selected when it is part of the selection.
- internal when it is selected, the parent piece is selected and the parent connection is selected.
- parent-piece-inclusive when the parent piece is selected.
- parent-piece-exclusive when the parent piece is not selected.
- parent-connection-inclusive when the parent connection is selected.
- parent-connection-exclusive when the parent connection is not selected.
- parent-inclusive when parent-piece-inclusive and parent-connection-inclusive.
- parent-exclusive when parent-piece-exclusive and parent-connection-exclusive.
- child-piece-inclusive when all child pieces are selected
- child-piece-mixed when some child pieces are selected and some are not selected.
- child-piece-exclusive when all child pieces are not selected
- child-connection-inclusive when all child connections are selected
- child-connection-mixed when some child connections are selected and some are not selected.
- child-connection-exclusive when all child connections are not selected
- child-inclusive when child-piece-inclusive and child-connection-inclusive.
- child-mixed when child-piece-mixed and child-connection-mixed.
- child-exclusive when child-piece-exclusive and child-connection-exclusive.

A connection is:

- selected when it is part of the selection.
- internal when the connection is selected and both pieces are selected.
- orphaned when the connection is selected and both pieces are not selected.


- parent-inclusive when the parent is selected.
- parent-exclusive when the parent is not selected.
- child-inclusive when the child piece is selected.
- child-exclusive when the child piece is not selected.

A design is:

- clumping when all pieces are interconnected.
- hanging when the design is clumping and has exactly one selected parent-exclusive connection along with the external parent piece.

Two connectors are similar when:

- Same name, compatible ports, same point, same direction
- Same name
- Compatible ports
- Similar point and similar direction

A bounding rectangle is the smallest rectangle (u,v domains) that can contain the selection. It uses the min/max of the set of center coords of pieces and the pieces of connections. For external connections, add the center of the external pieces to the set of center coords.




An anchor point is:

- middle: center of the bounding rectangle
- centroid: centroid of the bounding rectangle points
- bottomLeft: bottom-left of the bounding rectangle
- bottomRight: bottom-right of the bounding rectangle
- topLeft: top-left of the bounding rectangle
- topRight: top-right of the bounding rectangle


---

`copyDesign(design:Design, pieces:Guid[], connections:Guid[]): Design`:

- add every selected fixed pieces
- add every internal connected pieces
- add every internal connection
- add every selected parent-piece-exclusive parent-connection-inclusive piece with additional attributes: `semio.center` with the flat center of the piece and `semio.plane` with the flat plane of the piece.(these are pieces that are not internal but have the parent piece selected and the parent connection selected)

- add every orphaned connection, add every selected parent-exlusive child-inclusive connection, eadd every selected parent-inclusive child-exclusiv connection. Add all involved external pieces with additional attributes on the external pieces: `semio.piece.origin` set to `"external"`. (these are connections that are not internal but have either the parent or child piece selected)  



`pasteDesign(source:Design, target:Design, anchor: "original" | "middle" | "centroid" | "bottomLeft" | "bottomRight" | "topLeft" | "topRight" = "bottomLeft", coord?:Coord): DesignDiff`:

Questions to answer:
- No fixed piece as a case in the source... Does it get added to the boudning box for the paste?
- without coord and with coord difference ? connection infomation for the children and plane/u,v information for fixed pieces? How is the connection edit calculated?


Algorithm:

- compute the bounding rectangle, anchor point and paste vector

(pieces to add)
- add every copied selected fixed pieces (without coord,   with coord)
- add every copied internal connected pieces
- add every selected parent-piece-exclusive parent-connection-exclusive piece as fixed piece (selected pieces whose parent piece is not selected and parent connection is not selected become fixed pieces when pasted)

- when an added piece has attribute `semio.center` and `semio.plane`, it has to be connected to matching target pieces
the parent connection connected to this piece in the source design has to conenct to the matched target piece (replace external parent from source with matched target piece) and add an u,v offset to this connection .
- when an added piece has attribute `semio.center` and `semio.plane`, cannot be matched to a target piece, add the piece as a fixed piece with the same u,v coordinates as the original piece in the source design (using the `semio.center` attribute) and the same plane as the original piece in the source design (using the `semio.plane` attribute).



------ when an added piece has attribute `semio.piece.origin` set to `"external"`,  


- add every copied internal connection between added pieces
- add every copied orphaned connection and edit the connection to connect to the matched target pieces if found. If not found, do not add the connection.



- if the source design is hanging, paste it with `pasteHanging`

Paste Hanging


`pasteHanging(s): DesignDiff` pastes a hanging design …

Questions to answer:
- why call it hanging paste instead of target paste? (hanging design is only an edge case of normal paste )
- this example doesnt include a case where hanging wouldnt be pasted disconnectly without being connected to the target design  
- if the target design has selected pieces, paste it with `pastetargetted`
this case ?? 
- If there are no target selected pieces, the hanging connection is removed and the parent piece of the hanging connection in the source design becomes a fixed piece in the target design.


Terms to add :

hanging connection: the one selected parent-exclusive connection along within the hanging design

- add pieces and connections in the handing design to the target design
- conenct the hanging connection to target selected piece/s : hanging connection in the source design is connected to the target selected piece/s in the target design. If there are multiple target selected pieces, the hanging connection is duplicated the number of target selected pieces and each duplicate is connected to one of the target selected pieces. 







































version 2 : seperated pieces and connections

(pieces to add)
- add every copied selected fixed pieces 
- add every copied internal connected pieces
- add every selected parent-piece-exclusive parent-connection-exclusive piece as fixed piece (selected pieces whose parent piece is not selected and parent connection is not selected become fixed pieces when pasted)

- when an added piece has attribute `semio.center` and `semio.plane`, it has to be connected to matching target pieces
the parent connection connected to this piece in the source design has to conenct to the matched target piece (replace external parent from source with matched target piece) and add an u,v offset to this connection .
- when an added piece has attribute `semio.center` and `semio.plane`, cannot be matched to a target piece, add the piece as a fixed piece with the same u,v coordinates as the original piece in the source design (using the `semio.center` attribute) and the same plane as the original piece in the source design (using the `semio.plane` attribute).



- when an added piece has attribute `semio.piece.origin` set to `"external"`,  

(connections to add)
- add every copied internal connection between added pieces
- add every copied orphaned connection and edit the connection to connect to the matched target pieces if found. If not found, do not add the connection.