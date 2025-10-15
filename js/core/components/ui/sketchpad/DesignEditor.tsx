// #region Header

// DesignEditor.tsx

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion

import { DragEndEvent } from "@dnd-kit/core";
import { FC, useEffect, useMemo, useRef } from "react";
import { useHotkeys } from "react-hotkeys-hook";
import { useTranslation } from "react-i18next";

import { arrayMove } from "@dnd-kit/sortable";
import { Slider } from "@radix-ui/react-slider";
import { Connection, ReactFlowInstance, ReactFlowProvider } from "@xyflow/react";
import { Minus, Pin, Plus, Trash2 } from "lucide-react";
import { guid } from "../../../lib/utils";
import { Design, findConnectionInDesign, findPieceInDesign, findTypeInKit, Guid, ICON_WIDTH, Kit, parseDesignIdFromVariant, Piece } from "../../../semio";
import {
  DesignEditorFullscreenPanel,
  EditorType,
  useDesign,
  useDesignEditorCommands,
  useDesignEditorFullscreen,
  useDesignEditorSelection,
  useEditorType,
  useIsInDesignScope,
  useKit,
  useKitCommands,
  usePieces,
  useReplacableDesigns,
  useReplacableTypes,
  useSketchpad,
} from "../../../store";
import Combobox from "../Combobox";
import { Input } from "../Input";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "../Resizable";
import Stepper from "../Stepper";
import { Textarea } from "../Textarea";
import { SortableTreeItems, TreeContent, TreeItem, TreeSection } from "../Tree";
import DesignModel from "./DesignModel";
import Diagram from "./Diagram";
import { useAddPanelSection, useRemovePanelSection } from "./Navbar";
import { useDragDrop } from "./Sketchpad";
import { DesignAvatar, TypeAvatar } from "./Workbench";

const DesignSection: FC = () => {
  const isInDesignScope = useIsInDesignScope();
  if (!isInDesignScope) return null;
  return <DesignSectionForm />;
};

const DesignSectionForm: FC = () => {
  const { t } = useTranslation();
  const { startTransaction, finalizeTransaction, abortTransaction } = useDesignEditorCommands();
  const kit = useKit();
  const kitCommands = useKitCommands();
  const design = useDesign() as Design;

  const updateDesignField = (diff: any) => {
    kitCommands.updateDesign(design.guid, diff);
  };

  const handleChange = (updatedDesign: any) => {
    kitCommands.updateDesign(design.guid, updatedDesign);
  };

  const addLocation = () => {
    startTransaction();
    updateDesignField({ location: { guid: guid(), longitude: 0, latitude: 0 } });
    finalizeTransaction();
  };

  const removeLocation = () => {
    startTransaction();
    updateDesignField({ location: undefined });
    finalizeTransaction();
  };

  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input lazy label={t("design.name")} value={design.name} onLazyChange={(value) => updateDesignField({ name: value })} startTransaction={startTransaction} finalizeTransaction={finalizeTransaction} abortTransaction={abortTransaction} />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea
            lazy
            label={t("design.description")}
            value={design.description || ""}
            placeholder={t("design.descriptionPlaceholder")}
            onLazyChange={(value) => updateDesignField({ description: value })}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            label={t("design.icon")}
            value={design.icon || ""}
            placeholder={t("design.iconPlaceholder")}
            onLazyChange={(value) => updateDesignField({ icon: value })}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            label={t("design.image")}
            value={design.image || ""}
            placeholder={t("design.imagePlaceholder")}
            onLazyChange={(value) => updateDesignField({ image: value })}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            label={t("design.variant")}
            value={design.variant || ""}
            placeholder={t("design.variantPlaceholder")}
            onLazyChange={(value) => updateDesignField({ variant: value })}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            label={t("design.view")}
            value={design.view || ""}
            placeholder={t("design.viewPlaceholder")}
            onLazyChange={(value) => updateDesignField({ view: value })}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input lazy label={t("design.unit")} value={design.unit || ""} onLazyChange={(value) => updateDesignField({ unit: value })} startTransaction={startTransaction} finalizeTransaction={finalizeTransaction} abortTransaction={abortTransaction} />
        </TreeContent>
      </TreeItem>
      {design.location ? (
        <TreeSection
          label={t("design.location")}
          actions={[
            {
              icon: <Minus size={12} />,
              onClick: removeLocation,
              title: t("common.remove"),
            },
          ]}
        >
          <TreeItem>
            <TreeContent>
              <Stepper
                label={t("design.longitude")}
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
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <Stepper
                label={t("design.latitude")}
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
            </TreeContent>
          </TreeItem>
        </TreeSection>
      ) : (
        <TreeSection
          label={t("design.location")}
          actions={[
            {
              icon: <Plus size={12} />,
              onClick: addLocation,
              title: t("common.add"),
            },
          ]}
        ></TreeSection>
      )}
      {design.authors && design.authors.length > 0 ? (
        <TreeSection
          label={t("design.authors")}
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
              title: t("design.addAuthor"),
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
              <TreeItem key={`author-${index}`} label={author.name || `${t("design.author")} ${index + 1}`} sortable={true} sortableId={`author-${index}`} isDragHandle={true}>
                <TreeItem>
                  <TreeContent>
                    <Input
                      label={t("design.authorName")}
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
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      label={t("design.authorEmail")}
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
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
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
                      title={t("common.remove")}
                    >
                      <Trash2 size={12} />
                    </button>
                  </TreeContent>
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
          label={t("design.attributes")}
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
              title: t("design.addAttribute"),
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
                  <TreeContent>
                    <Input
                      label={t("design.attributeName")}
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
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      label={t("design.attributeValue")}
                      value={attribute.value || ""}
                      placeholder={t("design.attributeValuePlaceholder")}
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
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      label={t("design.attributeUnit")}
                      value={attribute.unit || ""}
                      placeholder={t("design.attributeUnitPlaceholder")}
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
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      label={t("design.attributeDefinition")}
                      value={attribute.definition || ""}
                      placeholder={t("design.attributeDefinitionPlaceholder")}
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
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
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
                      title={t("design.removeAttribute")}
                    >
                      <Trash2 size={12} />
                    </button>
                  </TreeContent>
                </TreeItem>
              </TreeItem>
            )}
          </SortableTreeItems>
        </TreeSection>
      ) : (
        <TreeSection
          label={t("design.attributes")}
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
              title: t("design.addAttribute"),
            },
          ]}
        ></TreeSection>
      )}
      <TreeItem label={t("design.metadata")}>
        {design.createdAt && (
          <TreeItem>
            <TreeContent>
              <Input label="Created" value={design.createdAt.toISOString().split("T")[0]} disabled />
            </TreeContent>
          </TreeItem>
        )}
        {design.updatedAt && (
          <TreeItem>
            <TreeContent>
              <Input label="Updated" value={design.updatedAt.toISOString().split("T")[0]} disabled />
            </TreeContent>
          </TreeItem>
        )}
        {design.pieces && design.pieces.length > 0 && (
          <TreeItem>
            <TreeContent>
              <Input label="Pieces" value={`${design.pieces.length} pieces`} disabled />
            </TreeContent>
          </TreeItem>
        )}
        {design.connections && design.connections.length > 0 && (
          <TreeItem>
            <TreeContent>
              <Input label="Connections" value={`${design.connections.length} connections`} disabled />
            </TreeContent>
          </TreeItem>
        )}
      </TreeItem>
    </>
  );
};

const PiecesSection: FC = () => {
  const isInDesignScope = useIsInDesignScope();
  if (!isInDesignScope) return null;
  return <PiecesSectionForm />;
};

const PiecesSectionForm: FC = () => {
  const { t } = useTranslation();
  const { startTransaction, finalizeTransaction, abortTransaction, executeCommand } = useDesignEditorCommands();
  const design = useDesign() as Design;
  // const metadata = usePiecesMetadata();
  const metadata = new Map();
  const pieces = usePieces();
  // const includedDesigns = useIncludedDesigns();

  // const includedDesignMap = useMemo(() => new Map(includedDesigns.map((d) => [d.id, d])), [includedDesigns]);

  const isSingle = pieces.length === 1;
  const piece = isSingle ? pieces[0] : null;

  // Check if we're dealing with design pieces
  const isDesignPiece = isSingle ? piece?.type.name === "design" : pieces.every((p) => p.type.name === "design");
  const hasDesignPieces = pieces.some((p) => p.type.name === "design");
  const hasMixedTypes = hasDesignPieces && pieces.some((p) => p.type.name !== "design");

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

  const selectedVariants = useMemo(() => [...new Set(pieces.map((p) => p.type.variant).filter((v): v is string => Boolean(v)))], [pieces]);
  const availableTypes = useReplacableTypes(pieces, isDesignPiece ? [] : selectedVariants);
  const availableTypeNames = useMemo(() => [...new Set(availableTypes.map((t) => t.name))], [availableTypes]);
  const allReplacableTypes = useReplacableTypes(pieces, []);
  const availableVariants = useMemo(
    () =>
      commonTypeName && !isDesignPiece
        ? [
          ...new Set(
            allReplacableTypes
              .filter((t) => t.name === commonTypeName)
              .map((t) => t.variant)
              .filter((v): v is string => Boolean(v)),
          ),
        ]
        : [],
    [commonTypeName, isDesignPiece, allReplacableTypes],
  );

  const availableDesigns = isDesignPiece && isSingle && piece ? useReplacableDesigns(piece) : [];
  const availableDesignNames = useMemo(() => [...new Set(availableDesigns.map((d) => d.name))], [availableDesigns]);

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
      } catch { }
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
        <TreeItem label={t("piece.mixedSelection", { count: pieces.length })} defaultOpen={true}>
          <TreeItem>
            <TreeContent>
              <p className="text-sm text-muted-foreground">{t("piece.mixedSelectionMessage")}</p>
            </TreeContent>
          </TreeItem>
        </TreeItem>
      ) : (
        <TreeItem
          label={isDesignPiece ? (isSingle ? t("piece.designPiece") : t("piece.multipleDesignPieces", { count: pieces.length })) : isSingle ? t("piece.piece") : t("piece.multiplePieces", { count: pieces.length })}
          defaultOpen={true}
          actions={
            hasUnfixedPieces
              ? [
                {
                  icon: <Pin size={12} />,
                  onClick: fixPieces,
                  title: isSingle ? t("piece.fixPiece") : t("piece.fixPieces"),
                },
              ]
              : undefined
          }
        >
          {isSingle && piece && (
            <TreeItem>
              <TreeContent>
                <Input label={t("piece.id")} value={getPieceId(piece)} disabled />
              </TreeContent>
            </TreeItem>
          )}

          {isDesignPiece ? (
            // Design piece fields
            <>
              <TreeItem>
                <TreeContent>
                  <Combobox
                    label={t("design.name")}
                    options={availableDesignNames.map((name) => ({
                      value: name,
                      label: name,
                    }))}
                    value={currentDesignId?.name || ""}
                    placeholder={t("common.selectDesign")}
                    onValueChange={handleDesignNameChange}
                  />
                </TreeContent>
              </TreeItem>
              {availableDesignVariants.length > 0 && (
                <TreeItem>
                  <TreeContent>
                    <Combobox
                      label={t("design.variant")}
                      options={availableDesignVariants.map((variant) => ({
                        value: variant,
                        label: variant,
                      }))}
                      value={currentDesignId?.variant || ""}
                      placeholder={t("common.selectVariant")}
                      onValueChange={handleDesignVariantChange}
                      allowClear={true}
                    />
                  </TreeContent>
                </TreeItem>
              )}
              {availableDesignViews.length > 0 && (
                <TreeItem>
                  <TreeContent>
                    <Combobox
                      label={t("design.view")}
                      options={availableDesignViews.map((view) => ({
                        value: view,
                        label: view,
                      }))}
                      value={currentDesignId?.view || ""}
                      placeholder={t("common.selectView")}
                      onValueChange={handleDesignViewChange}
                      allowClear={true}
                    />
                  </TreeContent>
                </TreeItem>
              )}
            </>
          ) : (
            // Regular piece fields
            <>
              <TreeItem>
                <TreeContent>
                  <Combobox
                    label={t("piece.type")}
                    options={availableTypeNames.map((name) => ({
                      value: name,
                      label: name,
                    }))}
                    value={isSingle && piece ? piece.type.name : commonTypeName || ""}
                    placeholder={!isSingle && commonTypeName === undefined ? t("common.mixedValues") : t("common.selectType")}
                    onValueChange={handleTypeNameChange}
                  />
                </TreeContent>
              </TreeItem>
              {(hasVariant || availableVariants.length > 0) && (
                <TreeItem>
                  <TreeContent>
                    <Combobox
                      label={t("type.variant")}
                      options={availableVariants.map((variant) => ({
                        value: variant,
                        label: variant,
                      }))}
                      value={isSingle && piece ? piece.type.variant || "" : commonTypeVariant || ""}
                      placeholder={!isSingle && commonTypeVariant === undefined ? t("common.mixedValues") : t("common.selectVariant")}
                      onValueChange={handleTypeVariantChange}
                      allowClear={true}
                    />
                  </TreeContent>
                </TreeItem>
              )}
            </>
          )}
        </TreeItem>
      )}
      {hasCenter && (
        <TreeItem label={t("piece.center")}>
          <TreeItem>
            <TreeContent>
              <Stepper
                label={t("common.x")}
                value={isSingle && piece ? piece.center?.x : commonCenterX}
                onChange={handleCenterXChange}
                onPointerDown={startTransaction}
                onPointerUp={finalizeTransaction}
                onPointerCancel={abortTransaction}
                step={0.1}
              />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <Stepper
                label={t("common.y")}
                value={isSingle && piece ? piece.center?.y : commonCenterY}
                onChange={handleCenterYChange}
                onPointerDown={startTransaction}
                onPointerUp={finalizeTransaction}
                onPointerCancel={abortTransaction}
                step={0.1}
              />
            </TreeContent>
          </TreeItem>
        </TreeItem>
      )}
      {hasPlane && (
        <TreeItem label={t("piece.plane")}>
          <TreeItem>
            <TreeContent>
              <Stepper
                label={t("common.x")}
                value={isSingle && piece ? piece.plane?.origin.x : commonPlaneOriginX}
                onChange={handlePlaneOriginXChange}
                onPointerDown={startTransaction}
                onPointerUp={finalizeTransaction}
                onPointerCancel={abortTransaction}
                step={0.1}
              />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <Stepper
                label={t("common.y")}
                value={isSingle && piece ? piece.plane?.origin.y : commonPlaneOriginY}
                onChange={handlePlaneOriginYChange}
                onPointerDown={startTransaction}
                onPointerUp={finalizeTransaction}
                onPointerCancel={abortTransaction}
                step={0.1}
              />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <Stepper
                label={t("common.z")}
                value={isSingle && piece ? piece.plane?.origin.z : commonPlaneOriginZ}
                onChange={handlePlaneOriginZChange}
                onPointerDown={startTransaction}
                onPointerUp={finalizeTransaction}
                onPointerCancel={abortTransaction}
                step={0.1}
              />
            </TreeContent>
          </TreeItem>
        </TreeItem>
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
  const isInDesignScope = useIsInDesignScope();
  if (!isInDesignScope) return null;
  return <ConnectionsSectionForm connections={connections} sectionLabel={sectionLabel} />;
};

const ConnectionsSectionForm: FC<{
  connections: ConnectionId[];
  sectionLabel?: string;
}> = ({ connections, sectionLabel }) => {
  const { t } = useTranslation();
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
    <TreeItem label={sectionLabel || (isSingle ? "Connection" : `Multiple Connections (${connections.length})`)} defaultOpen={true}>
      {isSingle && (
        <>
          <TreeItem>
            <TreeContent>
              <Input label="Connecting Piece ID" value={connection!.connecting.piece.id_} disabled />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <Input label="Connecting Port ID" value={connection!.connecting.port.id_} disabled />
            </TreeContent>
          </TreeItem>
          {connection!.connecting.designPiece && (
            <TreeItem>
              <TreeContent>
                <Input label="Connecting Design Piece ID" value={connection!.connecting.designPiece.id_} disabled />
              </TreeContent>
            </TreeItem>
          )}
          <TreeItem>
            <TreeContent>
              <Input label="Connected Piece ID" value={connection!.connected.piece.id_} disabled />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <Input label="Connected Port ID" value={connection!.connected.port.id_} disabled />
            </TreeContent>
          </TreeItem>
          {connection!.connected.designPiece && (
            <TreeItem>
              <TreeContent>
                <Input label="Connected Design Piece ID" value={connection!.connected.designPiece.id_} disabled />
              </TreeContent>
            </TreeItem>
          )}
        </>
      )}
      {!isSingle && (
        <TreeItem>
          <TreeContent>
            <p className="text-sm text-muted-foreground">Editing {connections.length} connections simultaneously</p>
          </TreeContent>
        </TreeItem>
      )}
      <TreeItem>
        <TreeContent>
          <Stepper label={t("connection.gap")} value={isSingle ? (connection!.gap ?? 0) : (commonGap ?? 0)} onChange={handleGapChange} onPointerDown={startTransaction} onPointerUp={finalizeTransaction} onPointerCancel={abortTransaction} step={0.1} />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Stepper
            label={t("connection.shift")}
            value={isSingle ? (connection!.shift ?? 0) : (commonShift ?? 0)}
            onChange={handleShiftChange}
            onPointerDown={startTransaction}
            onPointerUp={finalizeTransaction}
            onPointerCancel={abortTransaction}
            step={0.1}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Stepper
            label={t("connection.rise")}
            value={isSingle ? (connection!.rise ?? 0) : (commonRise ?? 0)}
            onChange={handleRiseChange}
            onPointerDown={startTransaction}
            onPointerUp={finalizeTransaction}
            onPointerCancel={abortTransaction}
            step={0.1}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <div className="flex flex-col gap-1">
            <label className="text-xs">{t("connection.rotation")}</label>
            <Slider
              value={[isSingle ? (connection!.rotation ?? 0) : (commonRotation ?? 0)]}
              onValueChange={([value]) => handleRotationChange(value)}
              onPointerDown={startTransaction}
              onPointerUp={finalizeTransaction}
              onPointerCancel={abortTransaction}
              min={-180}
              max={180}
              step={1}
            />
          </div>
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <div className="flex flex-col gap-1">
            <label className="text-xs">{t("connection.turn")}</label>
            <Slider
              value={[isSingle ? (connection!.turn ?? 0) : (commonTurn ?? 0)]}
              onValueChange={([value]) => handleTurnChange(value)}
              onPointerDown={startTransaction}
              onPointerUp={finalizeTransaction}
              onPointerCancel={abortTransaction}
              min={-180}
              max={180}
              step={1}
            />
          </div>
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <div className="flex flex-col gap-1">
            <label className="text-xs">{t("connection.tilt")}</label>
            <Slider
              value={[isSingle ? (connection!.tilt ?? 0) : (commonTilt ?? 0)]}
              onValueChange={([value]) => handleTiltChange(value)}
              onPointerDown={startTransaction}
              onPointerUp={finalizeTransaction}
              onPointerCancel={abortTransaction}
              min={-180}
              max={180}
              step={1}
            />
          </div>
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Stepper label="X Offset" value={isSingle ? (connection!.x ?? 0) : (commonXOffset ?? 0)} onChange={handleXOffsetChange} onPointerDown={startTransaction} onPointerUp={finalizeTransaction} onPointerCancel={abortTransaction} step={0.1} />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Stepper label="Y Offset" value={isSingle ? (connection!.y ?? 0) : (commonYOffset ?? 0)} onChange={handleYOffsetChange} onPointerDown={startTransaction} onPointerUp={finalizeTransaction} onPointerCancel={abortTransaction} step={0.1} />
        </TreeContent>
      </TreeItem>
    </TreeItem>
  );
};

const PortSection: FC<{ pieceId: PieceId; portId: PortId }> = ({ pieceId, portId }) => {
  const isInDesignScope = useIsInDesignScope();
  if (!isInDesignScope) return null;
  return <PortSectionForm pieceId={pieceId} portId={portId} />;
};

const PortSectionForm: FC<{ pieceId: PieceId; portId: PortId }> = ({ pieceId, portId }) => {
  const design = useDesign();
  const kit = useKit();

  const piece = (() => {
    try {
      return findPieceInDesign(design, pieceId);
    } catch {
      return null;
    }
  })();

  const type = piece ? findTypeInKit(kit, piece.type) : null;
  const port = type?.ports?.find((p: any) => p.id_ === portId);

  if (!piece || !type || !port) {
    return (
      <TreeItem label="Port" defaultOpen={true}>
        <TreeItem>
          <TreeContent>
            <p className="text-sm text-muted-foreground">Port not found</p>
          </TreeContent>
        </TreeItem>
      </TreeItem>
    );
  }

  return (
    <TreeItem label="Port" defaultOpen={true}>
      <Input label={t("piece.id")} value={port.id_ || "~default~"} disabled />
      {port.description && <Textarea label="Description" value={port.description} disabled />}
      {port.family && <Input label="Family" value={port.family} disabled />}
      {port.mandatory !== undefined && <Input label="Mandatory" value={port.mandatory ? "Yes" : "No"} disabled />}
      <Input label="Position" value={`(${port.point.x.toFixed(2)}, ${port.point.y.toFixed(2)}, ${port.point.z.toFixed(2)})`} disabled />
      <Input label="Direction" value={`(${port.direction.x.toFixed(2)}, ${port.direction.y.toFixed(2)}, ${port.direction.z.toFixed(2)})`} disabled />
      {port.compatibleFamilies &&
        port.compatibleFamilies.map((family: string) => (
          <TreeItem>
            <TreeContent>
              <Input label="Compatible Families" value={family} disabled />
            </TreeContent>
          </TreeItem>
        ))}
      {port.attributes &&
        port.attributes.map((attribute: any) => (
          <TreeItem>
            <TreeContent>
              <Input label={t("design.attributes")} value={`${attribute.key}: ${attribute.value || "N/A"} ${attribute.unit && `(${attribute.unit})`}`} disabled />
            </TreeContent>
          </TreeItem>
        ))}
    </TreeItem>
  );
};

export interface DesignEditorProps { }

const DesignEditor: FC<DesignEditorProps> = () => {
  const fullscreenPanel = useDesignEditorFullscreen();
  const { selectAll, deselectAll, deleteSelected, undo, redo, toggleDiagramFullscreen, toggleAccesslFullscreen, addPiece, startTransaction, finalizeTransaction } = useDesignEditorCommands();

  const selection = useDesignEditorSelection();
  const design = useDesign();
  const kit = useKit() as Kit;
  const editorSettings = useSketchpad((s) => s.editorSettings);
  const { activeDraggedType, activeDraggedDesign, setActiveDraggedType, setActiveDraggedDesign } = useDragDrop();

  const reactFlowInstanceRef = useRef<ReactFlowInstance | null>(null);

  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();

  useHotkeys("ctrl+a", () => selectAll());
  useHotkeys("ctrl+d", () => deselectAll());
  useHotkeys("delete", () => deleteSelected());
  useHotkeys("ctrl+z", () => undo());
  useHotkeys("ctrl+y", () => redo());
  useHotkeys("ctrl+shift+z", () => redo());

  const editorType = useEditorType();

  // Add/remove details panel sections based on selection
  useEffect(() => {
    // Only register sections if we're in the design editor
    if (editorType !== EditorType.DESIGN) return;

    const hasPieces = (selection.pieces || []).length > 0;
    const hasConnections = (selection.connections || []).length > 0;
    const hasPortSelected = selection.port !== undefined;
    const hasSelection = hasPieces || hasConnections || hasPortSelected;

    // Remove all details sections first
    removeSection("details", "design");
    removeSection("details", "port");
    removeSection("details", "pieces");
    removeSection("details", "connections");
    removeSection("details", "mixed");

    // Add appropriate sections based on selection
    if (!hasSelection) {
      addSection("details", {
        id: "design",
        label: "Design",
        order: 0,
        defaultOpen: true,
        content: () => <DesignSection />,
      });
    } else if (hasPortSelected) {
      const portPieceId = selection.port!.piece;
      const portId = selection.port!.port;
      addSection("details", {
        id: "port",
        label: "Port",
        order: 1,
        defaultOpen: true,
        content: () => <PortSection pieceId={portPieceId} portId={portId} />,
      });
    } else {
      if (hasPieces) {
        addSection("details", {
          id: "pieces",
          label: selection.pieces!.length === 1 ? "Piece" : `Pieces (${selection.pieces!.length})`,
          order: 2,
          defaultOpen: true,
          content: () => <PiecesSection />,
        });
      }
      if (hasConnections) {
        const conns = selection.connections!;
        addSection("details", {
          id: "connections",
          label: conns.length === 1 ? "Connection" : `Connections (${conns.length})`,
          order: 3,
          defaultOpen: true,
          content: () => <ConnectionsSection connections={conns} />,
        });
      }
      if (hasPieces && hasConnections) {
        addSection("details", {
          id: "mixed",
          label: "Mixed Selection",
          order: 4,
          defaultOpen: true,
          content: () => (
            <TreeItem>
              <TreeContent>
                <p className="text-sm text-muted-foreground">Select only pieces or only connections to edit details.</p>
              </TreeContent>
            </TreeItem>
          ),
        });
      }
    }

    return () => {
      removeSection("details", "design");
      removeSection("details", "port");
      removeSection("details", "pieces");
      removeSection("details", "connections");
      removeSection("details", "mixed");
    };
  }, [selection, addSection, removeSection, editorType]);

  // Add workbench sections
  useEffect(() => {
    const typesByName = (kit.types || []).reduce(
      (acc, type) => {
        if (!acc[type.name]) acc[type.name] = [];
        acc[type.name].push(type);
        return acc;
      },
      {} as Record<string, any[]>,
    );

    const designsByName = (kit.designs || []).reduce(
      (acc, design) => {
        if (!acc[design.name]) acc[design.name] = [];
        acc[design.name].push(design);
        return acc;
      },
      {} as Record<string, any[]>,
    );

    addSection("workbench", {
      id: "types",
      label: "Types",
      order: 0,
      defaultOpen: true,
      content: (
        <>
          {Object.entries(typesByName).map(([name, variants]) => (
            <TreeItem key={name} label={name} defaultOpen={false}>
              <TreeContent>
                <div className="grid grid-cols-[repeat(auto-fill,calc(var(--spacing)*8))] auto-rows-[calc(var(--spacing)*8)] justify-start gap-1 p-1">
                  {variants.map((type: any) => (
                    <TypeAvatar key={`${type.name}-${type.variant}`} type={type} showHoverCard={true} />
                  ))}
                </div>
              </TreeContent>
            </TreeItem>
          ))}
        </>
      ),
    });

    addSection("workbench", {
      id: "designs",
      label: "Designs",
      order: 1,
      defaultOpen: true,
      content: (
        <>
          {Object.entries(designsByName).map(([name, designs]) => (
            <TreeItem key={name} label={name} defaultOpen={false}>
              <TreeContent>
                <div className="grid grid-cols-[repeat(auto-fill,calc(var(--spacing)*8))] auto-rows-[calc(var(--spacing)*8)] justify-start gap-1 p-1">
                  {designs.map((design: any) => (
                    <DesignAvatar key={`${design.name}-${design.variant}-${design.view}`} design={design} showHoverCard={true} />
                  ))}
                </div>
              </TreeContent>
            </TreeItem>
          ))}
        </>
      ),
    });

    return () => {
      removeSection("workbench", "types");
      removeSection("workbench", "designs");
    };
  }, [kit, addSection, removeSection]);

  // Add settings section
  useEffect(() => {
    addSection("settings", {
      id: "design-editor-settings",
      label: "Design Editor",
      order: 100,
      defaultOpen: true,
      content: (
        <>
          <TreeItem>
            <TreeContent>
              <div className="flex flex-col gap-1">
                <label>Snappiness: {editorSettings.design?.snappiness}</label>
                <input type="range" min="0" max="20" value={editorSettings.design?.snappiness || 10} className="w-full" readOnly />
              </div>
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>Grid Size: {editorSettings.design?.gridSize || 24}px</TreeContent>
          </TreeItem>
        </>
      ),
    });

    return () => {
      removeSection("settings", "design-editor-settings");
    };
  }, [editorSettings, addSection, removeSection]);

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over, delta } = event;

    if (over && over.id === "diagram-drop-zone" && reactFlowInstanceRef.current) {
      if (!(event.activatorEvent instanceof PointerEvent)) {
        return;
      }

      const { x, y } = reactFlowInstanceRef.current.screenToFlowPosition({
        x: event.activatorEvent.clientX + delta.x,
        y: event.activatorEvent.clientY + delta.y,
      });

      if (activeDraggedType) {
        startTransaction();
        const piece = {
          guid: guid(),
          type: activeDraggedType.guid,
          center: { x: x / ICON_WIDTH - 0.5, y: -y / ICON_WIDTH + 0.5 },
        };
        addPiece(piece);
        finalizeTransaction();
      } else if (activeDraggedDesign) {
        startTransaction();
        const piece = {
          guid: guid(),
          design: activeDraggedDesign.guid,
          center: { x: x / ICON_WIDTH - 0.5, y: -y / ICON_WIDTH + 0.5 },
        };
        addPiece(piece);
        finalizeTransaction();
      }
    }

    setActiveDraggedType(null);
    setActiveDraggedDesign(null);
  };

  useEffect(() => {
    const listener = (e: Event) => {
      const customEvent = e as CustomEvent<DragEndEvent>;
      handleDragEnd(customEvent.detail);
    };
    window.addEventListener("design-drag-end", listener);
    return () => window.removeEventListener("design-drag-end", listener);
  }, [handleDragEnd]);

  return (
    <ReactFlowProvider>
      <ResizablePanelGroup direction="horizontal">
        <ResizablePanel defaultSize={fullscreenPanel === DesignEditorFullscreenPanel.Diagram ? 100 : 50} className={`${fullscreenPanel === DesignEditorFullscreenPanel.Accessl ? "hidden" : "block"}`} onDoubleClick={toggleDiagramFullscreen}>
          <Diagram reactFlowInstanceRef={reactFlowInstanceRef} />
        </ResizablePanel>
        <ResizableHandle className={`border-r ${fullscreenPanel !== DesignEditorFullscreenPanel.None ? "hidden" : "block"}`} />
        <ResizablePanel defaultSize={fullscreenPanel === DesignEditorFullscreenPanel.Accessl ? 100 : 50} className={`${fullscreenPanel === DesignEditorFullscreenPanel.Diagram ? "hidden" : "block"}`}>
          <DesignModel />
        </ResizablePanel>
      </ResizablePanelGroup>
    </ReactFlowProvider>
  );
};

export default DesignEditor;
