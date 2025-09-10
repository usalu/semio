import { arrayMove } from "@dnd-kit/sortable";
import { Minus, Pin, Plus, Trash2 } from "lucide-react";
import { FC, useState } from "react";

import { Input } from "@semio/js/components/ui/Input";
import { ScrollArea } from "@semio/js/components/ui/ScrollArea";
import { Slider } from "@semio/js/components/ui/Slider";
import Stepper from "@semio/js/components/ui/Stepper";
import { Textarea } from "@semio/js/components/ui/Textarea";
import { SortableTreeItems, Tree, TreeItem, TreeSection } from "@semio/js/components/ui/Tree";
import {
  Connection,
  ConnectionId,
  Design,
  findConnectionInDesign,
  findPieceInDesign,
  findReplacableDesignsForDesignPiece,
  findReplacableTypesForPieceInDesign,
  findReplacableTypesForPiecesInDesign,
  findTypeInKit,
  getIncludedDesigns,
  parseDesignIdFromVariant,
  Piece,
  PieceId,
  PortId,
} from "../../../semio";
import { useDesign, useDesignEditorCommands, useDesignEditorSelection, useKit, usePiecesMetadata } from "../../../store";
import Combobox from "../Combobox";
import { ResizablePanelProps } from "./DesignEditor";

interface DetailsProps extends ResizablePanelProps {}

const DesignSection: FC = () => {
  const { setDesign, startTransaction, finalizeTransaction, abortTransaction } = useDesignEditorCommands();
  const design = useDesign();

  const handleChange = (updatedDesign: Design) => {
    setDesign(updatedDesign);
  };

  const addLocation = () => {
    startTransaction();
    handleChange({ ...design, location: { longitude: 0, latitude: 0 } });
    finalizeTransaction();
  };

  const removeLocation = () => {
    startTransaction();
    handleChange({ ...design, location: undefined });
    finalizeTransaction();
  };

  return (
    <>
      <TreeSection label="Design" defaultOpen={true}>
        <TreeItem>
          <Input label="Name" value={design.name} onChange={(e) => handleChange({ ...design, name: e.target.value })} onFocus={startTransaction} onBlur={finalizeTransaction} />
        </TreeItem>
        <TreeItem>
          <Textarea label="Description" value={design.description || ""} placeholder="Enter design description..." onChange={(e) => handleChange({ ...design, description: e.target.value })} onFocus={startTransaction} onBlur={finalizeTransaction} />
        </TreeItem>
        <TreeItem>
          <Input label="Icon" value={design.icon || ""} placeholder="Emoji, name, or URL" onChange={(e) => handleChange({ ...design, icon: e.target.value })} onFocus={startTransaction} onBlur={finalizeTransaction} />
        </TreeItem>
        <TreeItem>
          <Input label="Image URL" value={design.image || ""} placeholder="URL to design image" onChange={(e) => handleChange({ ...design, image: e.target.value })} onFocus={startTransaction} onBlur={finalizeTransaction} />
        </TreeItem>
        <TreeItem>
          <Input label="Variant" value={design.variant || ""} placeholder="Design variant" onChange={(e) => handleChange({ ...design, variant: e.target.value })} onFocus={startTransaction} onBlur={finalizeTransaction} />
        </TreeItem>
        <TreeItem>
          <Input label="View" value={design.view || ""} placeholder="Design view" onChange={(e) => handleChange({ ...design, view: e.target.value })} onFocus={startTransaction} onBlur={finalizeTransaction} />
        </TreeItem>
        <TreeItem>
          <Input label="Unit" value={design.unit} onChange={(e) => handleChange({ ...design, unit: e.target.value })} onFocus={startTransaction} onBlur={finalizeTransaction} />
        </TreeItem>
      </TreeSection>
      {design.location ? (
        <TreeSection
          label="Location"
          actions={[
            {
              icon: <Minus size={12} />,
              onClick: removeLocation,
              title: "Remove location",
            },
          ]}
        >
          <TreeItem>
            <Stepper
              label="Longitude"
              value={design.location.longitude}
              onChange={(value) =>
                handleChange({
                  ...design,
                  location: { ...design.location!, longitude: value },
                })
              }
              onPointerDown={startTransaction}
              onPointerUp={finalizeTransaction}
              onPointerCancel={abortTransaction}
              step={0.000001}
            />
          </TreeItem>
          <TreeItem>
            <Stepper
              label="Latitude"
              value={design.location.latitude}
              onChange={(value) =>
                handleChange({
                  ...design,
                  location: { ...design.location!, latitude: value },
                })
              }
              onPointerDown={startTransaction}
              onPointerUp={finalizeTransaction}
              onPointerCancel={abortTransaction}
              step={0.000001}
            />
          </TreeItem>
        </TreeSection>
      ) : (
        <TreeSection
          label="Location"
          actions={[
            {
              icon: <Plus size={12} />,
              onClick: addLocation,
              title: "Add location",
            },
          ]}
        ></TreeSection>
      )}
      {design.authors && design.authors.length > 0 ? (
        <TreeSection
          label="Authors"
          actions={[
            {
              icon: <Plus size={12} />,
              onClick: () => {
                startTransaction();
                handleChange({
                  ...design,
                  authors: [...(design.authors || []), { name: "", email: "" }],
                });
                finalizeTransaction();
              },
              title: "Add author",
            },
          ]}
        >
          <SortableTreeItems
            items={design.authors.map((author: any, index: number) => ({
              ...author,
              id: `author-${index}`,
              index,
            }))}
            onReorder={(oldIndex, newIndex) => {
              startTransaction();
              handleChange({
                ...design,
                authors: arrayMove(design.authors!, oldIndex, newIndex),
              });
              finalizeTransaction();
            }}
          >
            {(author, index) => (
              <TreeItem key={`author-${index}`} label={author.name || `Author ${index + 1}`} sortable={true} sortableId={`author-${index}`} isDragHandle={true}>
                <TreeItem>
                  <Input
                    label="Name"
                    value={author.name}
                    onChange={(e) => {
                      const updatedAuthors = [...(design.authors || [])];
                      updatedAuthors[index] = {
                        ...author,
                        name: e.target.value,
                      };
                      handleChange({ ...design, authors: updatedAuthors });
                    }}
                    onFocus={startTransaction}
                    onBlur={finalizeTransaction}
                  />
                </TreeItem>
                <TreeItem>
                  <Input
                    label="Email"
                    value={author.email}
                    onChange={(e) => {
                      const updatedAuthors = [...(design.authors || [])];
                      updatedAuthors[index] = {
                        ...author,
                        email: e.target.value,
                      };
                      handleChange({ ...design, authors: updatedAuthors });
                    }}
                    onFocus={startTransaction}
                    onBlur={finalizeTransaction}
                  />
                </TreeItem>
                <TreeItem>
                  <button
                    onClick={() => {
                      startTransaction();
                      handleChange({
                        ...design,
                        authors: design.authors?.filter((_: any, i: number) => i !== index),
                      });
                      finalizeTransaction();
                    }}
                    className="text-destructive hover:text-destructive/80 p-1"
                    title="Remove author"
                  >
                    <Trash2 size={12} />
                  </button>
                </TreeItem>
              </TreeItem>
            )}
          </SortableTreeItems>
        </TreeSection>
      ) : (
        <TreeSection
          label="Authors"
          actions={[
            {
              icon: <Plus size={12} />,
              onClick: () => {
                startTransaction();
                handleChange({
                  ...design,
                  authors: [...(design.authors || []), { name: "", email: "" }],
                });
                finalizeTransaction();
              },
              title: "Add author",
            },
          ]}
        ></TreeSection>
      )}
      {design.attributes && design.attributes.length > 0 ? (
        <TreeSection
          label="Attributes"
          actions={[
            {
              icon: <Plus size={12} />,
              onClick: () => {
                startTransaction();
                handleChange({
                  ...design,
                  attributes: [...(design.attributes || []), { key: "" }],
                });
                finalizeTransaction();
              },
              title: "Add attribute",
            },
          ]}
        >
          <SortableTreeItems
            items={design.attributes.map((attribute: any, index: number) => ({
              ...attribute,
              id: `attribute-${index}`,
              index,
            }))}
            onReorder={(oldIndex, newIndex) => {
              startTransaction();
              handleChange({
                ...design,
                attributes: arrayMove(design.attributes!, oldIndex, newIndex),
              });
              finalizeTransaction();
            }}
          >
            {(attribute, index) => (
              <TreeItem key={`attribute-${index}`} label={attribute.key || `Attribute ${index + 1}`} sortable={true} sortableId={`attribute-${index}`} isDragHandle={true}>
                <TreeItem>
                  <Input
                    label="Name"
                    value={attribute.key}
                    onChange={(e) => {
                      const updatedAttributes = [...(design.attributes || [])];
                      updatedAttributes[index] = {
                        ...attribute,
                        key: e.target.value,
                      };
                      handleChange({ ...design, attributes: updatedAttributes });
                    }}
                    onFocus={startTransaction}
                    onBlur={finalizeTransaction}
                  />
                </TreeItem>
                <TreeItem>
                  <Input
                    label="Value"
                    value={attribute.value || ""}
                    placeholder="Optional value"
                    onChange={(e) => {
                      const updatedAttributes = [...(design.attributes || [])];
                      updatedAttributes[index] = {
                        ...attribute,
                        value: e.target.value,
                      };
                      handleChange({ ...design, attributes: updatedAttributes });
                    }}
                    onFocus={startTransaction}
                    onBlur={finalizeTransaction}
                  />
                </TreeItem>
                <TreeItem>
                  <Input
                    label="Unit"
                    value={attribute.unit || ""}
                    placeholder="Optional unit"
                    onChange={(e) => {
                      const updatedAttributes = [...(design.attributes || [])];
                      updatedAttributes[index] = {
                        ...attribute,
                        unit: e.target.value,
                      };
                      handleChange({ ...design, attributes: updatedAttributes });
                    }}
                    onFocus={startTransaction}
                    onBlur={finalizeTransaction}
                  />
                </TreeItem>
                <TreeItem>
                  <Input
                    label="Definition"
                    value={attribute.definition || ""}
                    placeholder="Optional definition (text or URL)"
                    onChange={(e) => {
                      const updatedAttributes = [...(design.attributes || [])];
                      updatedAttributes[index] = {
                        ...attribute,
                        definition: e.target.value,
                      };
                      handleChange({ ...design, attributes: updatedAttributes });
                    }}
                    onFocus={startTransaction}
                    onBlur={finalizeTransaction}
                  />
                </TreeItem>
                <TreeItem>
                  <button
                    onClick={() => {
                      startTransaction();
                      handleChange({
                        ...design,
                        attributes: design.attributes?.filter((_: any, i: number) => i !== index),
                      });
                      finalizeTransaction();
                    }}
                    className="text-destructive hover:text-destructive/80 p-1"
                    title="Remove attribute"
                  >
                    <Trash2 size={12} />
                  </button>
                </TreeItem>
              </TreeItem>
            )}
          </SortableTreeItems>
        </TreeSection>
      ) : (
        <TreeSection
          label="Attributes"
          actions={[
            {
              icon: <Plus size={12} />,
              onClick: () => {
                startTransaction();
                handleChange({
                  ...design,
                  attributes: [...(design.attributes || []), { key: "" }],
                });
                finalizeTransaction();
              },
              title: "Add attribute",
            },
          ]}
        ></TreeSection>
      )}
      <TreeSection label="Metadata">
        {design.created && (
          <TreeItem>
            <Input label="Created" value={design.created.toISOString().split("T")[0]} disabled />
          </TreeItem>
        )}
        {design.updated && (
          <TreeItem>
            <Input label="Updated" value={design.updated.toISOString().split("T")[0]} disabled />
          </TreeItem>
        )}
        {design.pieces && design.pieces.length > 0 && (
          <TreeItem>
            <Input label="Pieces" value={`${design.pieces.length} pieces`} disabled />
          </TreeItem>
        )}
        {design.connections && design.connections.length > 0 && (
          <TreeItem>
            <Input label="Connections" value={`${design.connections.length} connections`} disabled />
          </TreeItem>
        )}
      </TreeSection>
    </>
  );
};

const PiecesSection: FC<{ pieceIds: PieceId[] }> = ({ pieceIds }) => {
  const { setDesign, setPiece, setPieces, setConnection, startTransaction, finalizeTransaction, abortTransaction, executeCommand } = useDesignEditorCommands();
  const design = useDesign();
  const metadata = usePiecesMetadata();

  // Handle both regular pieces and synthetic design pieces (fixed and connected)
  const includedDesigns = getIncludedDesigns(design);
  const includedDesignMap = new Map(includedDesigns.map((d) => [d.id, d]));

  // Helper function to find parent connection for a design piece
  const findParentConnectionForDesignPiece = (pieceId: string): Connection | null => {
    const includedDesign = includedDesignMap.get(pieceId);
    if (!includedDesign || includedDesign.type !== "connected") {
      return null; // Fixed designs don't have parent connections
    }

    // For connected designs, find the actual parent connection using BFS metadata
    const pieceMetadata = metadata.get(pieceId);

    if (!pieceMetadata?.parentPieceId) {
      return null; // No parent found
    }

    // Find the connection between the parent piece and this design piece
    const parentPieceId = pieceMetadata.parentPieceId;

    // Look for a connection that connects the parent piece to this design piece
    // The connection should have the designId parameter set
    const parentConn = design.connections?.find((connection: Connection) => {
      const isParentConnecting = connection.connecting.piece.id_ === parentPieceId && connection.connected.designPiece?.id_ === includedDesign.designId.name;
      const isParentConnected = connection.connected.piece.id_ === parentPieceId && connection.connecting.designPiece?.id_ === includedDesign.designId.name;

      return isParentConnecting || isParentConnected;
    });

    return parentConn || null;
  };

  const pieces = pieceIds.map((id) => {
    try {
      // Try to find as regular piece first
      const foundPiece = findPieceInDesign(design, id);
      // Normalize the piece to ensure consistent ID structure
      return {
        ...foundPiece,
        id_: getPieceId(foundPiece),
      };
    } catch {
      // Check if it's a design piece (either fixed or connected)
      const pieceIdString = getPieceId({ id_: id } as any);
      const includedDesign = includedDesignMap.get(pieceIdString);
      if (includedDesign) {
        // Create a synthetic piece that matches the design
        return {
          id_: pieceIdString,
          type: {
            name: "design",
            variant:
              includedDesign.type === "fixed"
                ? `${includedDesign.designId.name}${includedDesign.designId.variant ? `-${includedDesign.designId.variant}` : ""}${includedDesign.designId.view ? `-${includedDesign.designId.view}` : ""}`
                : includedDesign.designId.name,
          },
          center: includedDesign.center,
          plane: includedDesign.plane,
          description: `${includedDesign.type === "fixed" ? "Fixed" : "Clustered"} design: ${includedDesign.designId.name}`,
        };
      }

      // If still not found, create a fallback synthetic piece instead of throwing an error
      console.warn(`Piece ${pieceIdString} not found in pieces or includedDesigns. Creating fallback piece.`);
      return {
        id_: pieceIdString,
        type: {
          name: "unknown",
          variant: "",
        },
        description: `Unknown piece: ${pieceIdString}`,
      };
    }
  });

  const isSingle = pieceIds.length === 1;
  const piece = isSingle ? pieces[0] : null;

  // Check if we're dealing with design pieces
  const isDesignPiece = isSingle ? piece?.type.name === "design" : pieces.every((p) => p.type.name === "design");
  const hasDesignPieces = pieces.some((p) => p.type.name === "design");
  const hasMixedTypes = hasDesignPieces && pieces.some((p) => p.type.name !== "design");

  // Safety check - if no valid pieces found, show a message
  if (pieces.length === 0 || pieces.every((p) => p.type.name === "unknown")) {
    return (
      <TreeSection label="Pieces" defaultOpen={true}>
        <TreeItem>
          <p className="text-sm text-muted-foreground">No valid pieces found in selection.</p>
        </TreeItem>
      </TreeSection>
    );
  }

  const getCommonValue = <T,>(getter: (piece: Piece) => T | undefined): T | undefined => {
    const values = pieces.map(getter).filter((v) => v !== undefined);
    if (values.length === 0) return undefined;
    const firstValue = values[0];
    return values.every((v) => JSON.stringify(v) === JSON.stringify(firstValue)) ? firstValue : undefined;
  };

  const getPieceId = (piece: Piece): string => {
    return typeof piece.id_ === "string" ? piece.id_ : (piece.id_ as any).id_;
  };

  const handleTypeNameChange = (value: string) => {
    if (!piece || piece.type.name === "unknown") return;

    startTransaction();
    if (isSingle) {
      setPiece({ ...piece!, id_: piece!.id_, type: { ...piece!.type, name: value } } as Piece);
    } else {
      const updatedPieces = pieces.map((piece) => ({
        ...piece,
        id_: getPieceId(piece),
        type: { ...piece.type, name: value },
      }));
      setPieces(updatedPieces as Piece[]);
    }
    finalizeTransaction();
  };

  const handleTypeVariantChange = (value: string) => {
    if (!piece || piece.type.name === "unknown") return;

    startTransaction();
    if (isSingle) {
      setPiece({ ...piece!, id_: piece!.id_, type: { ...piece!.type, variant: value } } as Piece);
    } else {
      const updatedPieces = pieces.map((piece) => ({
        ...piece,
        id_: piece.id_,
        type: { ...piece.type, variant: value },
      }));
      setPieces(updatedPieces as Piece[]);
    }
    finalizeTransaction();
  };

  // Design-specific handlers
  const handleDesignNameChange = (value: string) => {
    if (!isDesignPiece || !piece || piece.type.name === "unknown") return;

    startTransaction();
    if (isSingle) {
      const pieceId = getPieceId(piece!);
      const includedDesign = includedDesignMap.get(pieceId);

      if (includedDesign && includedDesign.type === "fixed") {
        // Handle fixed design piece - update the fixedDesigns array
        const newDesignId = {
          name: value,
          variant: includedDesign.designId.variant,
          view: includedDesign.designId.view,
        };

        // Find and update the fixed design entry
        const updatedFixedDesigns = (design.fixedDesigns || []).map((fd: any) => {
          if (fd.designId.name === includedDesign.designId.name && (fd.designId.variant || undefined) === (includedDesign.designId.variant || undefined) && (fd.designId.view || undefined) === (includedDesign.designId.view || undefined)) {
            return { ...fd, designId: newDesignId };
          }
          return fd;
        });

        // Update the design with new fixedDesigns array
        const updatedDesign = { ...design, fixedDesigns: updatedFixedDesigns };
        setDesign(updatedDesign);
      } else if (includedDesign && includedDesign.type === "connected") {
        // Connected designs cannot be renamed from here - they are clustered designs
        console.warn("Connected design pieces cannot be renamed - they represent clustered designs");
      } else {
        // Handle regular design piece
        const currentDesignId = parseDesignIdFromVariant(piece!.type.variant || "");
        const newVariant = `${value}${currentDesignId.variant ? `-${currentDesignId.variant}` : ""}${currentDesignId.view ? `-${currentDesignId.view}` : ""}`;
        setPiece({ ...piece!, type: { ...piece!.type, variant: newVariant } });
      }
    } else {
      const updatedPieces = pieces.map((piece) => {
        const currentDesignId = parseDesignIdFromVariant(piece.type.variant || "");
        const newVariant = `${value}${currentDesignId.variant ? `-${currentDesignId.variant}` : ""}${currentDesignId.view ? `-${currentDesignId.view}` : ""}`;
        return { ...piece, type: { ...piece.type, variant: newVariant } };
      });
      setPieces(updatedPieces);
    }
    finalizeTransaction();
  };

  const handleDesignVariantChange = (value: string) => {
    if (!isDesignPiece || !piece || piece.type.name === "unknown") return;

    startTransaction();
    if (isSingle) {
      const pieceId = getPieceId(piece!);
      const includedDesign = includedDesignMap.get(pieceId);

      if (includedDesign && includedDesign.type === "fixed") {
        // Handle fixed design piece
        const newDesignId = {
          name: includedDesign.designId.name,
          variant: value || undefined,
          view: includedDesign.designId.view,
        };

        const updatedFixedDesigns = (design.fixedDesigns || []).map((fd: any) => {
          if (fd.designId.name === includedDesign.designId.name && (fd.designId.variant || undefined) === (includedDesign.designId.variant || undefined) && (fd.designId.view || undefined) === (includedDesign.designId.view || undefined)) {
            return { ...fd, designId: newDesignId };
          }
          return fd;
        });

        const updatedDesign = { ...design, fixedDesigns: updatedFixedDesigns };
        setDesign(updatedDesign);
      } else if (includedDesign && includedDesign.type === "connected") {
        // Connected designs cannot have their variants changed
        console.warn("Connected design pieces cannot have variants changed - they represent clustered designs");
      } else {
        // Handle regular design piece
        const currentDesignId = parseDesignIdFromVariant(piece!.type.variant || "");
        const newVariant = `${currentDesignId.name}${value ? `-${value}` : ""}${currentDesignId.view ? `-${currentDesignId.view}` : ""}`;
        setPiece({ ...piece!, type: { ...piece!.type, variant: newVariant } });
      }
    } else {
      const updatedPieces = pieces.map((piece) => {
        const currentDesignId = parseDesignIdFromVariant(piece.type.variant || "");
        const newVariant = `${currentDesignId.name}${value ? `-${value}` : ""}${currentDesignId.view ? `-${currentDesignId.view}` : ""}`;
        return { ...piece, type: { ...piece.type, variant: newVariant } };
      });
      setPieces(updatedPieces);
    }
    finalizeTransaction();
  };

  const handleDesignViewChange = (value: string) => {
    if (!isDesignPiece || !piece || piece.type.name === "unknown") return;

    startTransaction();
    if (isSingle) {
      const pieceId = getPieceId(piece!);
      const includedDesign = includedDesignMap.get(pieceId);

      if (includedDesign && includedDesign.type === "fixed") {
        // Handle fixed design piece
        const newDesignId = {
          name: includedDesign.designId.name,
          variant: includedDesign.designId.variant,
          view: value || undefined,
        };

        const updatedFixedDesigns = (design.fixedDesigns || []).map((fd: any) => {
          if (fd.designId.name === includedDesign.designId.name && (fd.designId.variant || undefined) === (includedDesign.designId.variant || undefined) && (fd.designId.view || undefined) === (includedDesign.designId.view || undefined)) {
            return { ...fd, designId: newDesignId };
          }
          return fd;
        });

        const updatedDesign = { ...design, fixedDesigns: updatedFixedDesigns };
        setDesign(updatedDesign);
      } else if (includedDesign && includedDesign.type === "connected") {
        // Connected designs cannot have their views changed
        console.warn("Connected design pieces cannot have views changed - they represent clustered designs");
      } else if (piece) {
        // Handle regular design piece
        const currentDesignId = parseDesignIdFromVariant(piece.type.variant || "");
        const newVariant = `${currentDesignId.name}${currentDesignId.variant ? `-${currentDesignId.variant}` : ""}${value ? `-${value}` : ""}`;
        setPiece({ ...piece, type: { ...piece.type, variant: newVariant } });
      }
    } else {
      const updatedPieces = pieces.map((piece) => {
        const currentDesignId = parseDesignIdFromVariant(piece.type.variant || "");
        const newVariant = `${currentDesignId.name}${currentDesignId.variant ? `-${currentDesignId.variant}` : ""}${value ? `-${value}` : ""}`;
        return { ...piece, type: { ...piece.type, variant: newVariant } };
      });
      setPieces(updatedPieces);
    }
    finalizeTransaction();
  };

  const fixPieces = async () => {
    try {
      await executeCommand("fix-selected-pieces");
    } catch (error) {
      console.error("Failed to fix pieces:", error);
    }
  };

  const handleCenterXChange = (value: number) => {
    if (isSingle) {
      if (!piece || piece.type.name === "unknown") return;
      setPiece(piece!.center ? { ...piece!, center: { ...piece!.center, x: value } } : piece!);
    } else {
      const updatedPieces = pieces.map((piece) => (piece.center ? { ...piece, center: { ...piece.center, x: value } } : piece));
      setPieces(updatedPieces);
    }
  };

  const handleCenterYChange = (value: number) => {
    if (isSingle) {
      if (!piece || piece.type.name === "unknown") return;
      setPiece(piece!.center ? { ...piece!, center: { ...piece!.center, y: value } } : piece!);
    } else {
      const updatedPieces = pieces.map((piece) => (piece.center ? { ...piece, center: { ...piece.center, y: value } } : piece));
      setPieces(updatedPieces);
    }
  };

  const handlePlaneOriginXChange = (value: number) => {
    if (isSingle) {
      if (!piece || piece.type.name === "unknown") return;
      setPiece(
        piece!.plane
          ? {
              ...piece!,
              plane: {
                ...piece!.plane,
                origin: { ...piece!.plane.origin, x: value },
              },
            }
          : piece!,
      );
    } else {
      const updatedPieces = pieces.map((piece) =>
        piece.plane
          ? {
              ...piece,
              plane: {
                ...piece.plane,
                origin: { ...piece.plane.origin, x: value },
              },
            }
          : piece,
      );
      setPieces(updatedPieces);
    }
  };

  const handlePlaneOriginYChange = (value: number) => {
    if (isSingle) {
      if (!piece || piece.type.name === "unknown") return;
      setPiece(
        piece!.plane
          ? {
              ...piece!,
              plane: {
                ...piece!.plane,
                origin: { ...piece!.plane.origin, y: value },
              },
            }
          : piece!,
      );
    } else {
      const updatedPieces = pieces.map((piece) =>
        piece.plane
          ? {
              ...piece,
              plane: {
                ...piece.plane,
                origin: { ...piece.plane.origin, y: value },
              },
            }
          : piece,
      );
      setPieces(updatedPieces);
    }
  };

  const handlePlaneOriginZChange = (value: number) => {
    if (isSingle) {
      if (!piece || piece.type.name === "unknown") return;
      setPiece(
        piece!.plane
          ? {
              ...piece!,
              plane: {
                ...piece!.plane,
                origin: { ...piece!.plane.origin, z: value },
              },
            }
          : piece!,
      );
    } else {
      const updatedPieces = pieces.map((piece) =>
        piece.plane
          ? {
              ...piece,
              plane: {
                ...piece.plane,
                origin: { ...piece.plane.origin, z: value },
              },
            }
          : piece,
      );
      setPieces(updatedPieces);
    }
  };

  const commonTypeName = getCommonValue((p) => p.type.name);
  const commonTypeVariant = getCommonValue((p) => p.type.variant);
  const commonCenterX = getCommonValue((p) => p.center?.x);
  const commonCenterY = getCommonValue((p) => p.center?.y);
  const commonPlaneOriginX = getCommonValue((p) => p.plane?.origin.x);
  const commonPlaneOriginY = getCommonValue((p) => p.plane?.origin.y);
  const commonPlaneOriginZ = getCommonValue((p) => p.plane?.origin.z);

  const hasCenter = pieces.some((p) => p.center);
  const hasPlane = pieces.some((p) => p.plane);
  const hasVariant = pieces.some((p) => p.type.variant);
  const hasUnfixedPieces = pieces.some((p) => !p.plane || !p.center);

  // For regular pieces
  const selectedVariants = [...new Set(pieces.map((p) => p.type.variant).filter((v): v is string => Boolean(v)))];
  const availableTypes = !isDesignPiece ? (isSingle ? findReplacableTypesForPieceInDesign(kit, designId, pieceIds[0], selectedVariants) : findReplacableTypesForPiecesInDesign(kit, designId, pieceIds, selectedVariants)) : [];
  const availableTypeNames = [...new Set(availableTypes.map((t) => t.name))];
  const availableVariants =
    commonTypeName && !isDesignPiece
      ? [
          ...new Set(
            (isSingle ? findReplacableTypesForPieceInDesign(kit, designId, pieceIds[0]) : findReplacableTypesForPiecesInDesign(kit, designId, pieceIds))
              .filter((t) => t.name === commonTypeName)
              .map((t) => t.variant)
              .filter((v): v is string => Boolean(v)),
          ),
        ]
      : [];

  // For design pieces
  const availableDesigns = isDesignPiece && isSingle ? findReplacableDesignsForDesignPiece(kit, designId, piece!) : [];
  const availableDesignNames = [...new Set(availableDesigns.map((d) => d.name))];

  // Parse current design ID for design pieces
  const currentDesignId = isDesignPiece && isSingle ? parseDesignIdFromVariant(piece!.type.variant || "") : null;

  // Get available design variants and views
  const availableDesignVariants = currentDesignId
    ? [
        ...new Set(
          availableDesigns
            .filter((d) => d.name === currentDesignId.name)
            .map((d) => d.variant)
            .filter((v): v is string => Boolean(v)),
        ),
      ]
    : [];

  const availableDesignViews = currentDesignId
    ? [
        ...new Set(
          availableDesigns
            .filter((d) => d.name === currentDesignId.name && (d.variant || "") === (currentDesignId.variant || ""))
            .map((d) => d.view)
            .filter((v): v is string => Boolean(v)),
        ),
      ]
    : [];

  let parentConnection: Connection | null = null;
  let parentConnections: Connection[] = [];

  if (isSingle && piece) {
    const pieceMetadata = metadata.get(getPieceId(piece));
    if (pieceMetadata?.parentPieceId) {
      try {
        parentConnection = findConnectionInDesign(design, {
          connected: { piece: { id_: getPieceId(piece) } },
          connecting: { piece: { id_: pieceMetadata.parentPieceId } },
        });
      } catch {}
    }

    // For design pieces, also check for external connections
    if (isDesignPiece && piece.type.name === "design") {
      const parentConn = findParentConnectionForDesignPiece(getPieceId(piece));
      if (parentConn) {
        parentConnection = parentConn;
      }
    }
  } else if (!isSingle) {
    // For multiple pieces, find all their parent connections
    parentConnections = pieces
      .map((piece) => {
        const pieceMetadata = metadata.get(getPieceId(piece));
        if (pieceMetadata?.parentPieceId) {
          try {
            return findConnectionInDesign(design, {
              connected: { piece: { id_: getPieceId(piece) } },
              connecting: { piece: { id_: pieceMetadata.parentPieceId } },
            });
          } catch {
            return null;
          }
        }

        // For design pieces, also check for external connections
        if (piece.type.name === "design") {
          const parentConn = findParentConnectionForDesignPiece(getPieceId(piece));
          if (parentConn) {
            return parentConn;
          }
        }

        return null;
      })
      .filter((conn): conn is Connection => conn !== null);
  }

  return (
    <>
      {hasMixedTypes ? (
        <TreeSection label={`Mixed Selection (${pieceIds.length})`} defaultOpen={true}>
          <TreeItem>
            <p className="text-sm text-muted-foreground">Selection contains both design pieces and regular pieces. Select only design pieces or only regular pieces to edit properties.</p>
          </TreeItem>
        </TreeSection>
      ) : (
        <TreeSection
          label={isDesignPiece ? (isSingle ? "Design Piece" : `Multiple Design Pieces (${pieceIds.length})`) : isSingle ? "Piece" : `Multiple Pieces (${pieceIds.length})`}
          defaultOpen={true}
          actions={
            hasUnfixedPieces
              ? [
                  {
                    icon: <Pin size={12} />,
                    onClick: fixPieces,
                    title: isSingle ? "Fix piece" : "Fix pieces",
                  },
                ]
              : undefined
          }
        >
          {isSingle && piece && (
            <TreeItem>
              <Input label="ID" value={getPieceId(piece)} disabled />
            </TreeItem>
          )}

          {isDesignPiece ? (
            // Design piece fields
            <>
              <TreeItem>
                <Combobox
                  label="Design Name"
                  options={availableDesignNames.map((name) => ({
                    value: name,
                    label: name,
                  }))}
                  value={currentDesignId?.name || ""}
                  placeholder="Select design"
                  onValueChange={handleDesignNameChange}
                />
              </TreeItem>
              {availableDesignVariants.length > 0 && (
                <TreeItem>
                  <Combobox
                    label="Design Variant"
                    options={availableDesignVariants.map((variant) => ({
                      value: variant,
                      label: variant,
                    }))}
                    value={currentDesignId?.variant || ""}
                    placeholder="Select variant"
                    onValueChange={handleDesignVariantChange}
                    allowClear={true}
                  />
                </TreeItem>
              )}
              {availableDesignViews.length > 0 && (
                <TreeItem>
                  <Combobox
                    label="Design View"
                    options={availableDesignViews.map((view) => ({
                      value: view,
                      label: view,
                    }))}
                    value={currentDesignId?.view || ""}
                    placeholder="Select view"
                    onValueChange={handleDesignViewChange}
                    allowClear={true}
                  />
                </TreeItem>
              )}
            </>
          ) : (
            // Regular piece fields
            <>
              <TreeItem>
                <Combobox
                  label="Type"
                  options={availableTypeNames.map((name) => ({
                    value: name,
                    label: name,
                  }))}
                  value={isSingle && piece ? piece.type.name : commonTypeName || ""}
                  placeholder={!isSingle && commonTypeName === undefined ? "Mixed values" : "Select type"}
                  onValueChange={handleTypeNameChange}
                />
              </TreeItem>
              {(hasVariant || availableVariants.length > 0) && (
                <TreeItem>
                  <Combobox
                    label="Variant"
                    options={availableVariants.map((variant) => ({
                      value: variant,
                      label: variant,
                    }))}
                    value={isSingle && piece ? piece.type.variant || "" : commonTypeVariant || ""}
                    placeholder={!isSingle && commonTypeVariant === undefined ? "Mixed values" : "Select variant"}
                    onValueChange={handleTypeVariantChange}
                    allowClear={true}
                  />
                </TreeItem>
              )}
            </>
          )}
        </TreeSection>
      )}
      {hasCenter && (
        <TreeSection label="Center">
          <TreeItem>
            <Stepper label="X" value={isSingle && piece ? piece.center?.x : commonCenterX} onChange={handleCenterXChange} onPointerDown={startTransaction} onPointerUp={finalizeTransaction} onPointerCancel={abortTransaction} step={0.1} />
          </TreeItem>
          <TreeItem>
            <Stepper label="Y" value={isSingle && piece ? piece.center?.y : commonCenterY} onChange={handleCenterYChange} onPointerDown={startTransaction} onPointerUp={finalizeTransaction} onPointerCancel={abortTransaction} step={0.1} />
          </TreeItem>
        </TreeSection>
      )}
      {hasPlane && (
        <TreeSection label="Plane">
          <TreeSection label="Origin" defaultOpen={true}>
            <TreeItem>
              <Stepper
                label="X"
                value={isSingle && piece ? piece.plane?.origin.x : commonPlaneOriginX}
                onChange={handlePlaneOriginXChange}
                onPointerDown={startTransaction}
                onPointerUp={finalizeTransaction}
                onPointerCancel={abortTransaction}
                step={0.1}
              />
            </TreeItem>
            <TreeItem>
              <Stepper
                label="Y"
                value={isSingle && piece ? piece.plane?.origin.y : commonPlaneOriginY}
                onChange={handlePlaneOriginYChange}
                onPointerDown={startTransaction}
                onPointerUp={finalizeTransaction}
                onPointerCancel={abortTransaction}
                step={0.1}
              />
            </TreeItem>
            <TreeItem>
              <Stepper
                label="Z"
                value={isSingle && piece ? piece.plane?.origin.z : commonPlaneOriginZ}
                onChange={handlePlaneOriginZChange}
                onPointerDown={startTransaction}
                onPointerUp={finalizeTransaction}
                onPointerCancel={abortTransaction}
                step={0.1}
              />
            </TreeItem>
          </TreeSection>
        </TreeSection>
      )}
      {(parentConnection || parentConnections.length > 0) && (
        <div style={{ marginTop: "0.5rem" }}>
          <ConnectionsSection connections={isSingle && parentConnection ? [parentConnection] : parentConnections} sectionLabel={isSingle ? "Parent Connection" : `Parent Connections (${parentConnections.length})`} />
        </div>
      )}
    </>
  );
};

const ConnectionsSection: FC<{
  connections: ConnectionId[];
  sectionLabel?: string;
}> = ({ connections, sectionLabel }) => {
  const { setConnection, setConnections, startTransaction, finalizeTransaction, abortTransaction } = useDesignEditorCommands();
  const design = useDesign();
  const connectionObjects = connections.map((conn) => {
    // The conn is already a ConnectionId, but we need to create a full ConnectionId to query
    const connectionId = {
      connecting: {
        piece: conn.connecting.piece,
        // Include port and designPiece only if they exist in the connection
        ...((conn.connecting as any).port && { port: (conn.connecting as any).port }),
        ...((conn.connecting as any).designPiece && { designPiece: (conn.connecting as any).designPiece }),
      },
      connected: {
        piece: conn.connected.piece,
        ...((conn.connected as any).port && { port: (conn.connected as any).port }),
        ...((conn.connected as any).designPiece && { designPiece: (conn.connected as any).designPiece }),
      },
    };

    // Try to find the connection in the design
    return findConnectionInDesign(design, connectionId);
  });

  const isSingle = connections.length === 1;
  const connection = isSingle ? connectionObjects[0] : null;

  const getCommonValue = <T,>(getter: (connection: Connection) => T | undefined): T | undefined => {
    const values = connectionObjects.map(getter).filter((v) => v !== undefined);
    if (values.length === 0) return undefined;
    const firstValue = values[0];
    return values.every((v) => JSON.stringify(v) === JSON.stringify(firstValue)) ? firstValue : undefined;
  };

  const handleChange = (updatedConnection: Connection) => setConnection(updatedConnection);

  const handleGapChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, gap: value });
    else setConnections(connectionObjects.map((connection) => ({ ...connection, gap: value })));
  };

  const handleShiftChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, shift: value });
    else
      setConnections(
        connectionObjects.map((connection) => ({
          ...connection,
          shift: value,
        })),
      );
  };

  const handleRiseChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, rise: value });
    else setConnections(connectionObjects.map((connection) => ({ ...connection, rise: value })));
  };

  const handleXOffsetChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, x: value });
    else setConnections(connectionObjects.map((connection) => ({ ...connection, x: value })));
  };

  const handleYOffsetChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, y: value });
    else setConnections(connectionObjects.map((connection) => ({ ...connection, y: value })));
  };

  const handleRotationChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, rotation: value });
    else
      setConnections(
        connectionObjects.map((connection) => ({
          ...connection,
          rotation: value,
        })),
      );
  };

  const handleTurnChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, turn: value });
    else setConnections(connectionObjects.map((connection) => ({ ...connection, turn: value })));
  };

  const handleTiltChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, tilt: value });
    else setConnections(connectionObjects.map((connection) => ({ ...connection, tilt: value })));
  };

  const commonGap = getCommonValue((c) => c.gap);
  const commonShift = getCommonValue((c) => c.shift);
  const commonRise = getCommonValue((c) => c.rise);
  const commonXOffset = getCommonValue((c) => c.x);
  const commonYOffset = getCommonValue((c) => c.y);
  const commonRotation = getCommonValue((c) => c.rotation);
  const commonTurn = getCommonValue((c) => c.turn);
  const commonTilt = getCommonValue((c) => c.tilt);

  return (
    <TreeSection label={sectionLabel || (isSingle ? "Connection" : `Multiple Connections (${connections.length})`)} defaultOpen={true}>
      {isSingle && (
        <>
          <TreeItem label="Connecting">
            <TreeItem>
              <Input label="Piece ID" value={connection!.connecting.piece.id_} disabled />
            </TreeItem>
            <TreeItem>
              <Input label="Port ID" value={connection!.connecting.port.id_} disabled />
            </TreeItem>
            {connection!.connecting.designPiece && (
              <TreeItem>
                <Input label="Design Piece ID" value={connection!.connecting.designPiece.id_} disabled />
              </TreeItem>
            )}
          </TreeItem>
          <TreeItem label="Connected">
            <TreeItem>
              <Input label="Piece ID" value={connection!.connected.piece.id_} disabled />
            </TreeItem>
            <TreeItem>
              <Input label="Port ID" value={connection!.connected.port.id_} disabled />
            </TreeItem>
            {connection!.connected.designPiece && (
              <TreeItem>
                <Input label="Design Piece ID" value={connection!.connected.designPiece.id_} disabled />
              </TreeItem>
            )}
          </TreeItem>
        </>
      )}
      {!isSingle && (
        <TreeItem>
          <p className="text-sm text-muted-foreground">Editing {connections.length} connections simultaneously</p>
        </TreeItem>
      )}
      <TreeItem label="Plane">
        <TreeItem label="Translation" defaultOpen={true}>
          <TreeItem>
            <Stepper label="Gap" value={isSingle ? (connection!.gap ?? 0) : (commonGap ?? 0)} onChange={handleGapChange} onPointerDown={startTransaction} onPointerUp={finalizeTransaction} onPointerCancel={abortTransaction} step={0.1} />
          </TreeItem>
          <TreeItem>
            <Stepper label="Shift" value={isSingle ? (connection!.shift ?? 0) : (commonShift ?? 0)} onChange={handleShiftChange} onPointerDown={startTransaction} onPointerUp={finalizeTransaction} onPointerCancel={abortTransaction} step={0.1} />
          </TreeItem>
          <TreeItem>
            <Stepper label="Rise" value={isSingle ? (connection!.rise ?? 0) : (commonRise ?? 0)} onChange={handleRiseChange} onPointerDown={startTransaction} onPointerUp={finalizeTransaction} onPointerCancel={abortTransaction} step={0.1} />
          </TreeItem>
        </TreeItem>
        <TreeItem label="Orientation">
          <TreeItem>
            <Slider
              label="Rotation"
              value={[isSingle ? (connection!.rotation ?? 0) : (commonRotation ?? 0)]}
              onValueChange={([value]) => handleRotationChange(value)}
              onPointerDown={startTransaction}
              onPointerUp={finalizeTransaction}
              onPointerCancel={abortTransaction}
              min={-180}
              max={180}
              step={1}
            />
          </TreeItem>
          <TreeItem>
            <Slider
              label="Turn"
              value={[isSingle ? (connection!.turn ?? 0) : (commonTurn ?? 0)]}
              onValueChange={([value]) => handleTurnChange(value)}
              onPointerDown={startTransaction}
              onPointerUp={finalizeTransaction}
              onPointerCancel={abortTransaction}
              min={-180}
              max={180}
              step={1}
            />
          </TreeItem>
          <TreeItem>
            <Slider
              label="Tilt"
              value={[isSingle ? (connection!.tilt ?? 0) : (commonTilt ?? 0)]}
              onValueChange={([value]) => handleTiltChange(value)}
              onPointerDown={startTransaction}
              onPointerUp={finalizeTransaction}
              onPointerCancel={abortTransaction}
              min={-180}
              max={180}
              step={1}
            />
          </TreeItem>
        </TreeItem>
      </TreeItem>
      <TreeItem label="Diagram">
        <TreeItem>
          <Stepper label="X Offset" value={isSingle ? (connection!.x ?? 0) : (commonXOffset ?? 0)} onChange={handleXOffsetChange} onPointerDown={startTransaction} onPointerUp={finalizeTransaction} onPointerCancel={abortTransaction} step={0.1} />
        </TreeItem>
        <TreeItem>
          <Stepper label="Y Offset" value={isSingle ? (connection!.y ?? 0) : (commonYOffset ?? 0)} onChange={handleYOffsetChange} onPointerDown={startTransaction} onPointerUp={finalizeTransaction} onPointerCancel={abortTransaction} step={0.1} />
        </TreeItem>
      </TreeItem>
    </TreeSection>
  );
};

const PortSection: FC<{ pieceId: PieceId; portId: PortId }> = ({ pieceId, portId }) => {
  const design = useDesign();
  const kit = useKit();

  // Find the piece using the same pattern as in PiecesSection
  const piece = (() => {
    try {
      return findPieceInDesign(design, pieceId);
    } catch {
      return null; // Piece not found
    }
  })();

  const type = piece ? findTypeInKit(kit, piece.type) : null;
  const port = type?.ports?.find((p: any) => p.id_ === portId);

  if (!piece || !type || !port) {
    return (
      <TreeSection label="Port" defaultOpen={true}>
        <TreeItem>
          <p className="text-sm text-muted-foreground">Port not found</p>
        </TreeItem>
      </TreeSection>
    );
  }

  return (
    <TreeSection label="Port" defaultOpen={true}>
      <Input label="ID" value={port.id_ || "~default~"} disabled />
      {port.description && <Textarea label="Description" value={port.description} disabled />}
      {port.family && <Input label="Family" value={port.family} disabled />}
      {port.mandatory !== undefined && <Input label="Mandatory" value={port.mandatory ? "Yes" : "No"} disabled />}
      <Input label="Position" value={`(${port.point.x.toFixed(2)}, ${port.point.y.toFixed(2)}, ${port.point.z.toFixed(2)})`} disabled />
      <Input label="Direction" value={`(${port.direction.x.toFixed(2)}, ${port.direction.y.toFixed(2)}, ${port.direction.z.toFixed(2)})`} disabled />
      {port.compatibleFamilies &&
        port.compatibleFamilies.map((family: string) => (
          <TreeItem>
            <Input label="Compatible Families" value={family} disabled />
          </TreeItem>
        ))}
      {port.attributes &&
        port.attributes.map((attribute: any) => (
          <TreeItem>
            <Input label="Attributes" value={`${attribute.key}: ${attribute.value || "N/A"} ${attribute.unit && `(${attribute.unit})`}`} disabled />
          </TreeItem>
        ))}
    </TreeSection>
  );
};

const Details: FC<DetailsProps> = ({ visible, onWidthChange, width }) => {
  if (!visible) return null;
  const [isResizeHovered, setIsResizeHovered] = useState(false);
  const [isResizing, setIsResizing] = useState(false);

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);

    const startX = e.clientX;
    const startWidth = width;

    const handleMouseMove = (e: MouseEvent) => {
      const newWidth = startWidth - (e.clientX - startX);
      if (newWidth >= 150 && newWidth <= 500) {
        onWidthChange?.(newWidth);
      }
    };

    const handleMouseUp = () => {
      setIsResizing(false);
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseup", handleMouseUp);
    };

    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseup", handleMouseUp);
  };

  const selection = useDesignEditorSelection();

  const hasPieces = (selection.pieces || []).length > 0;
  const hasConnections = (selection.connections || []).length > 0;
  const hasPortSelected = selection.port !== undefined;
  const hasSelection = hasPieces || hasConnections || hasPortSelected;

  return (
    <div
      className={`absolute top-4 right-4 bottom-4 z-20 bg-background-level-2 text-foreground border min-w-0 overflow-hidden
                ${isResizing || isResizeHovered ? "border-l-primary" : "border-l"}`}
      style={{ width: `${width}px` }}
    >
      <ScrollArea className="h-full">
        <div className="p-1 overflow-hidden min-w-0">
          <Tree className="min-w-0 overflow-hidden">
            {!hasSelection && <DesignSection />}
            {hasPortSelected && <PortSection pieceId={selection.port!.piece.id_} portId={selection.port!.port.id_} />}
            {hasPieces && !hasPortSelected && <PiecesSection pieceIds={selection.pieces || []} />}
            {hasConnections && !hasPortSelected && <ConnectionsSection connections={selection.connections || []} />}
            {hasPieces && hasConnections && !hasPortSelected && (
              <TreeSection label="Mixed Selection" defaultOpen={true}>
                <TreeItem>
                  <p className="text-sm text-muted-foreground">Select only pieces or only connections to edit details.</p>
                </TreeItem>
              </TreeSection>
            )}
          </Tree>
        </div>
      </ScrollArea>
      <div className="absolute top-0 bottom-0 left-0 w-1 cursor-ew-resize" onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />
    </div>
  );
};

export default Details;
