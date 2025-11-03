// #region Header

// Details.tsx

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

import { arrayMove } from "@dnd-kit/sortable";
import { Link2Off, Minus, Plus } from "lucide-react";
import { FC, useMemo } from "react";
import { useTranslation } from "react-i18next";
import { SortableTreeItems, TreeContent, TreeItem } from "../../../../elements/aggregation/Tree";
import { Button } from "../../../../elements/input/Button";
import Combobox from "../../../../elements/input/Combobox";
import { Input } from "../../../../elements/input/Input";
import { Slider } from "../../../../elements/input/Slider";
import Stepper from "../../../../elements/input/Stepper";
import { Textarea } from "../../../../elements/input/Textarea";
import { Connection, ConnectionDiff, Design, Guid, Kit, Piece, findDesignInKit, findPieceInDesign, findTypeInKit, guid } from "../../../../semio";
import { useFlatPieceCenter, useFlatPiecePlane, useIsConnectedPiece, usePieceParentConnection, usePiecesMetadata } from "../../../kits";
import { useDesign, useIsInDesignScope, useKit, useKitCommands, usePiecesFromIds, useReplacableDesigns, useReplacableTypes, useTooltip } from "../../../store";
import { useDesignAppCommands, useDesignAppSelection } from "../store";

export const DesignSection: FC = () => {
  const isInDesignScope = useIsInDesignScope();
  if (!isInDesignScope) return null;
  return <DesignSectionForm />;
};

const DesignSectionForm: FC = () => {
  const { t } = useTranslation();
  const tooltip = useTooltip();
  const { startTransaction, finalizeTransaction, abortTransaction } = useDesignAppCommands();
  const kit = useKit();
  const kitCommands = useKitCommands();
  const design = useDesign() as Design;

  const updateDesignField = (origin: string, diff: any) => {
    kitCommands.updateDesign(origin, design.guid, diff);
  };

  const handleChange = (origin: string, updatedDesign: any) => {
    kitCommands.updateDesign(origin, design.guid, updatedDesign);
  };

  const addLocation = () => {
    startTransaction("semio.sketchpad.app.design.panel.details.location.add");
    updateDesignField("semio.sketchpad.app.design.panel.details.location.add", { location: { guid: guid(), longitude: 0, latitude: 0 } });
    finalizeTransaction("semio.sketchpad.app.design.panel.details.location.add");
  };

  const removeLocation = () => {
    startTransaction("semio.sketchpad.app.design.panel.details.location.remove");
    updateDesignField("semio.sketchpad.app.design.panel.details.location.remove", { location: undefined });
    finalizeTransaction("semio.sketchpad.app.design.panel.details.location.remove");
  };

  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.design.panel.details.section.design.name"
            value={design.name}
            onLazyChange={(value) => updateDesignField("semio.sketchpad.app.design.panel.details.section.design.name", { name: value })}
            startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.design.name")}
            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.design.name")}
            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.design.name")}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea
            lazy
            id="semio.sketchpad.app.design.panel.details.section.design.description"
            value={design.description || ""}
            placeholder={t("semio.sketchpad.app.design.descriptionPlaceholder")}
            onLazyChange={(value) => updateDesignField("semio.sketchpad.app.design.panel.details.section.design.description", { description: value })}
            startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.design.description")}
            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.design.description")}
            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.design.description")}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.design.panel.details.section.design.icon"
            value={design.icon || ""}
            placeholder={t("semio.sketchpad.app.design.iconPlaceholder")}
            onLazyChange={(value) => updateDesignField("semio.sketchpad.app.design.panel.details.section.design.icon", { icon: value })}
            startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.design.icon")}
            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.design.icon")}
            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.design.icon")}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.design.panel.details.section.design.image"
            value={design.image || ""}
            placeholder={t("semio.sketchpad.app.design.imagePlaceholder")}
            onLazyChange={(value) => updateDesignField("semio.sketchpad.app.design.panel.details.section.design.image", { image: value })}
            startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.design.image")}
            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.design.image")}
            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.design.image")}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.design.panel.details.section.design.variant"
            value={design.variant || ""}
            placeholder={t("semio.sketchpad.app.design.variantPlaceholder")}
            onLazyChange={(value) => updateDesignField("semio.sketchpad.app.design.panel.details.section.design.variant", { variant: value })}
            startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.design.variant")}
            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.design.variant")}
            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.design.variant")}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.design.panel.details.section.design.view"
            value={design.view || ""}
            placeholder={t("semio.sketchpad.app.design.viewPlaceholder")}
            onLazyChange={(value) => updateDesignField("semio.sketchpad.app.design.panel.details.section.design.view", { view: value })}
            startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.design.view")}
            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.design.view")}
            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.design.view")}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.design.panel.details.section.design.unit"
            value={design.unit || ""}
            onLazyChange={(value) => updateDesignField("semio.sketchpad.app.design.panel.details.section.design.unit", { unit: value })}
            startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.design.unit")}
            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.design.unit")}
            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.design.unit")}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      {design.location ? (
        <TreeItem
          label={t("semio.sketchpad.app.design.location")}
          actions={[
            {
              icon: <Minus />,
              onClick: removeLocation,
              id: "semio.sketchpad.common.remove",
            },
          ]}
        >
          <TreeItem>
            <TreeContent>
              <Stepper
                id="semio.sketchpad.app.design.panel.details.section.location.longitude"
                value={design.location.longitude}
                onChange={(value) =>
                  handleChange("semio.sketchpad.app.design.panel.details.section.location.longitude", {
                    ...design,
                    location: { ...design.location!, longitude: value },
                  })
                }
                startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.location.longitude")}
                finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.location.longitude")}
                abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.location.longitude")}
                step={0.000001}
              />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <Stepper
                id="semio.sketchpad.app.design.panel.details.section.location.latitude"
                value={design.location.latitude}
                onChange={(value) =>
                  handleChange("semio.sketchpad.app.design.panel.details.section.location.latitude", {
                    ...design,
                    location: { ...design.location!, latitude: value },
                  })
                }
                startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.location.latitude")}
                finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.location.latitude")}
                abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.location.latitude")}
                step={0.000001}
              />
            </TreeContent>
          </TreeItem>
        </TreeItem>
      ) : (
        <TreeItem
          label={t("semio.sketchpad.app.design.location")}
          actions={[
            {
              icon: <Plus />,
              onClick: addLocation,
              id: "semio.sketchpad.common.add",
            },
          ]}
        />
      )}
      <TreeItem
        label={t("semio.sketchpad.app.design.authors")}
        actions={[
          {
            icon: <Plus />,
            onClick: () => {
              const origin = "semio.sketchpad.app.design.panel.details.authors.add";
              startTransaction(origin);
              updateDesignField(origin, {
                authors: [...(design.authors || []), { name: "", email: "" }],
              });
              finalizeTransaction(origin);
            },
            id: "semio.sketchpad.common.add",
          },
        ]}
      >
        {design.authors && design.authors.length > 0 && (
          <SortableTreeItems
            items={(design.authors || []).map((author: any, index: number) => ({
              ...author,
              id: `author-${index}`,
              index,
            }))}
            onReorder={(oldIndex, newIndex) => {
              const origin = "semio.sketchpad.app.design.panel.details.authors.reorder";
              startTransaction(origin);
              updateDesignField(origin, {
                authors: arrayMove(design.authors!, oldIndex, newIndex),
              });
              finalizeTransaction(origin);
            }}
          >
            {(author, index) => (
              <TreeItem
                key={`author-${index}`}
                label={author.name || `${t("semio.sketchpad.app.design.author")} ${index + 1}`}
                sortable={true}
                sortableId={`author-${index}`}
                isDragHandle={true}
                actions={[
                  {
                    icon: <Minus />,
                    onClick: () => {
                      const origin = "semio.sketchpad.app.design.panel.details.authors.remove";
                      startTransaction(origin);
                      updateDesignField(origin, {
                        authors: design.authors?.filter((_: any, i: number) => i !== index),
                      });
                      finalizeTransaction(origin);
                    },
                    id: "semio.sketchpad.common.remove",
                  },
                ]}
              >
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.design.panel.details.section.authors.name"
                      value={author.name}
                      onChange={(e) => {
                        const updatedAuthors = [...(design.authors || [])];
                        updatedAuthors[index] = {
                          ...author,
                          name: e.target.value,
                        };
                        updateDesignField("semio.sketchpad.app.design.panel.details.section.authors.name", { authors: updatedAuthors });
                      }}
                      onFocus={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.authors.name")}
                      onBlur={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.authors.name")}
                      showLabel
                    />
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.design.panel.details.section.authors.email"
                      value={author.email}
                      onChange={(e) => {
                        const updatedAuthors = [...(design.authors || [])];
                        updatedAuthors[index] = {
                          ...author,
                          email: e.target.value,
                        };
                        updateDesignField("semio.sketchpad.app.design.panel.details.section.authors.email", { authors: updatedAuthors });
                      }}
                      onFocus={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.authors.email")}
                      onBlur={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.authors.email")}
                      showLabel
                    />
                  </TreeContent>
                </TreeItem>
              </TreeItem>
            )}
          </SortableTreeItems>
        )}
      </TreeItem>
      <TreeItem
        label={t("semio.sketchpad.app.design.attributes")}
        actions={[
          {
            icon: <Plus />,
            onClick: () => {
              const origin = "semio.sketchpad.app.design.panel.details.attributes.add";
              startTransaction(origin);
              updateDesignField(origin, {
                attributes: [...(design.attributes || []), { key: "" }],
              });
              finalizeTransaction(origin);
            },
            id: "semio.sketchpad.common.add",
          },
        ]}
      >
        {design.attributes && design.attributes.length > 0 && (
          <SortableTreeItems
            items={(design.attributes || []).map((attribute: any, index: number) => ({
              ...attribute,
              id: `attribute-${index}`,
              index,
            }))}
            onReorder={(oldIndex, newIndex) => {
              const origin = "semio.sketchpad.app.design.panel.details.attributes.reorder";
              startTransaction(origin);
              updateDesignField(origin, {
                attributes: arrayMove(design.attributes!, oldIndex, newIndex),
              });
              finalizeTransaction(origin);
            }}
          >
            {(attribute, index) => (
              <TreeItem
                key={`attribute-${index}`}
                label={attribute.key || `${t("semio.sketchpad.app.design.attribute")} ${index + 1}`}
                sortable={true}
                sortableId={`attribute-${index}`}
                isDragHandle={true}
                actions={[
                  {
                    icon: <Minus />,
                    onClick: () => {
                      const origin = "semio.sketchpad.app.design.panel.details.attributes.remove";
                      startTransaction(origin);
                      updateDesignField(origin, {
                        attributes: design.attributes?.filter((_: any, i: number) => i !== index),
                      });
                      finalizeTransaction(origin);
                    },
                    id: "semio.sketchpad.common.remove",
                  },
                ]}
              >
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.design.panel.details.section.attributes.name"
                      value={attribute.key}
                      onChange={(e) => {
                        const updatedAttributes = [...(design.attributes || [])];
                        updatedAttributes[index] = {
                          ...attribute,
                          key: e.target.value,
                        };
                        updateDesignField("semio.sketchpad.app.design.panel.details.section.attributes.name", { attributes: updatedAttributes });
                      }}
                      onFocus={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.attributes.name")}
                      onBlur={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.attributes.name")}
                      showLabel
                    />
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.design.panel.details.section.attributes.value"
                      value={attribute.value || ""}
                      placeholder={t("semio.sketchpad.app.design.attributeValuePlaceholder")}
                      onChange={(e) => {
                        const updatedAttributes = [...(design.attributes || [])];
                        updatedAttributes[index] = {
                          ...attribute,
                          value: e.target.value,
                        };
                        updateDesignField("semio.sketchpad.app.design.panel.details.section.attributes.value", { attributes: updatedAttributes });
                      }}
                      onFocus={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.attributes.value")}
                      onBlur={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.attributes.value")}
                      showLabel
                    />
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.design.panel.details.section.attributes.unit"
                      value={attribute.unit || ""}
                      placeholder={t("semio.sketchpad.app.design.attributeUnitPlaceholder")}
                      onChange={(e) => {
                        const updatedAttributes = [...(design.attributes || [])];
                        updatedAttributes[index] = {
                          ...attribute,
                          unit: e.target.value,
                        };
                        updateDesignField("semio.sketchpad.app.design.panel.details.section.attributes.unit", { attributes: updatedAttributes });
                      }}
                      onFocus={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.attributes.unit")}
                      onBlur={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.attributes.unit")}
                      showLabel
                    />
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.design.panel.details.section.attributes.definition"
                      value={attribute.definition || ""}
                      placeholder={t("semio.sketchpad.app.design.attributeDefinitionPlaceholder")}
                      onChange={(e) => {
                        const updatedAttributes = [...(design.attributes || [])];
                        updatedAttributes[index] = {
                          ...attribute,
                          definition: e.target.value,
                        };
                        updateDesignField("semio.sketchpad.app.design.panel.details.section.attributes.definition", { attributes: updatedAttributes });
                      }}
                      onFocus={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.attributes.definition")}
                      onBlur={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.attributes.definition")}
                      showLabel
                    />
                  </TreeContent>
                </TreeItem>
              </TreeItem>
            )}
          </SortableTreeItems>
        )}
      </TreeItem>
      {design.createdAt && (
        <TreeItem>
          <TreeContent>
            <Input
              id="semio.sketchpad.app.design.panel.details.section.design.createdAt"
              value={(() => {
                const date = design.createdAt;
                if (date instanceof Date) return date.toISOString().split("T")[0];
                if (typeof date === "string") return (date as string).split("T")[0];
                return "";
              })()}
              disabled
              showLabel
            />
          </TreeContent>
        </TreeItem>
      )}
      {design.updatedAt && (
        <TreeItem>
          <TreeContent>
            <Input
              id="semio.sketchpad.app.design.panel.details.section.design.updatedAt"
              value={(() => {
                const date = design.updatedAt;
                if (date instanceof Date) return date.toISOString().split("T")[0];
                if (typeof date === "string") return (date as string).split("T")[0];
                return "";
              })()}
              disabled
              showLabel
            />
          </TreeContent>
        </TreeItem>
      )}
    </>
  );
};

export const PiecesSection: FC = () => {
  const isInDesignScope = useIsInDesignScope();
  if (!isInDesignScope) return null;
  return <PiecesSectionForm />;
};

const PiecesSectionForm: FC = () => {
  const { t } = useTranslation();
  const { startTransaction, finalizeTransaction, abortTransaction, updatePiece, updatePieces } = useDesignAppCommands();
  const design = useDesign() as Design;
  const kit = useKit() as Kit;
  // const metadata = usePiecesMetadata();
  const metadata = new Map();
  const selection = useDesignAppSelection();
  const pieces = usePiecesFromIds(selection.pieces || []);
  // const includedDesigns = useIncludedDesigns();

  // const includedDesignMap = useMemo(() => new Map(includedDesigns.map((d) => [d.id, d])), [includedDesigns]);

  const isSingle = pieces.length === 1;
  const piece = isSingle ? pieces[0] : null;

  // Check if we're dealing with design pieces
  const isDesignPiece = isSingle ? typeof piece?.type === "string" && piece?.type === "design" : pieces.every((p) => typeof p.type === "string" && p.type === "design");
  const hasDesignPieces = pieces.some((p) => typeof p.type === "string" && p.type === "design");
  const hasMixedTypes = hasDesignPieces && pieces.some((p) => typeof p.type === "string" && p.type !== "design");

  const getCommonValue = <T,>(getter: (piece: Piece) => T | undefined): T | undefined => {
    const values = pieces.map(getter).filter((v) => v !== undefined);
    if (values.length === 0) return undefined;
    const firstValue = values[0];
    return values.every((v) => JSON.stringify(v) === JSON.stringify(firstValue)) ? firstValue : undefined;
  };

  const getPieceId = (piece: Piece): string => {
    return piece.guid;
  };

  const handleTypeNameChange = (value: string) => {
    // TODO: Implement using updatePiece/updatePieces commands
  };

  const handleTypeVariantChange = (value: string) => {
    // TODO: Implement using updatePiece/updatePieces commands
  };

  const handleDesignNameChange = (value: string) => {
    // TODO: Implement using updatePiece/updatePieces commands
  };

  const handleDesignVariantChange = (value: string) => {
    // TODO: Implement using updatePiece/updatePieces commands
  };

  const handleDesignViewChange = (value: string) => {
    // TODO: Implement using updatePiece/updatePieces commands
  };

  const fixPieces = async () => {
    // TODO: Implement using execute command
  };

  const handleCenterXChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.center.x";
    if (isSingle && piece) {
      updatePiece(origin, piece.guid, { center: { x: value, y: piece.center?.y ?? 0 } });
    } else {
      const updates = pieces.map((p) => ({ id: p.guid, diff: { center: { x: value, y: p.center?.y ?? 0 } } }));
      updatePieces(origin, updates);
    }
  };

  const handleCenterYChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.center.y";
    if (isSingle && piece) {
      updatePiece(origin, piece.guid, { center: { x: piece.center?.x ?? 0, y: value } });
    } else {
      const updates = pieces.map((p) => ({ id: p.guid, diff: { center: { x: p.center?.x ?? 0, y: value } } }));
      updatePieces(origin, updates);
    }
  };

  const handlePlaneOriginXChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.plane.origin.x";
    if (isSingle && piece && piece.plane) {
      updatePiece(origin, piece.guid, { plane: { ...piece.plane, origin: { ...piece.plane.origin, x: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: p.guid, diff: { plane: { ...p.plane!, origin: { ...p.plane!.origin, x: value } } } }));
      updatePieces(origin, updates);
    }
  };

  const handlePlaneOriginYChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.plane.origin.y";
    if (isSingle && piece && piece.plane) {
      updatePiece(origin, piece.guid, { plane: { ...piece.plane, origin: { ...piece.plane.origin, y: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: p.guid, diff: { plane: { ...p.plane!, origin: { ...p.plane!.origin, y: value } } } }));
      updatePieces(origin, updates);
    }
  };

  const handlePlaneOriginZChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.plane.origin.z";
    if (isSingle && piece && piece.plane) {
      updatePiece(origin, piece.guid, { plane: { ...piece.plane, origin: { ...piece.plane.origin, z: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: p.guid, diff: { plane: { ...p.plane!, origin: { ...p.plane!.origin, z: value } } } }));
      updatePieces(origin, updates);
    }
  };

  const handlePlaneXAxisXChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.x";
    if (isSingle && piece && piece.plane) {
      updatePiece(origin, piece.guid, { plane: { ...piece.plane, xAxis: { ...piece.plane.xAxis, x: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: p.guid, diff: { plane: { ...p.plane!, xAxis: { ...p.plane!.xAxis, x: value } } } }));
      updatePieces(origin, updates);
    }
  };

  const handlePlaneXAxisYChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.y";
    if (isSingle && piece && piece.plane) {
      updatePiece(origin, piece.guid, { plane: { ...piece.plane, xAxis: { ...piece.plane.xAxis, y: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: p.guid, diff: { plane: { ...p.plane!, xAxis: { ...p.plane!.xAxis, y: value } } } }));
      updatePieces(origin, updates);
    }
  };

  const handlePlaneXAxisZChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.z";
    if (isSingle && piece && piece.plane) {
      updatePiece(origin, piece.guid, { plane: { ...piece.plane, xAxis: { ...piece.plane.xAxis, z: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: p.guid, diff: { plane: { ...p.plane!, xAxis: { ...p.plane!.xAxis, z: value } } } }));
      updatePieces(origin, updates);
    }
  };

  const handlePlaneYAxisXChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.x";
    if (isSingle && piece && piece.plane) {
      updatePiece(origin, piece.guid, { plane: { ...piece.plane, yAxis: { ...piece.plane.yAxis, x: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: p.guid, diff: { plane: { ...p.plane!, yAxis: { ...p.plane!.yAxis, x: value } } } }));
      updatePieces(origin, updates);
    }
  };

  const handlePlaneYAxisYChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.y";
    if (isSingle && piece && piece.plane) {
      updatePiece(origin, piece.guid, { plane: { ...piece.plane, yAxis: { ...piece.plane.yAxis, y: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: p.guid, diff: { plane: { ...p.plane!, yAxis: { ...p.plane!.yAxis, y: value } } } }));
      updatePieces(origin, updates);
    }
  };

  const handlePlaneYAxisZChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.z";
    if (isSingle && piece && piece.plane) {
      updatePiece(origin, piece.guid, { plane: { ...piece.plane, yAxis: { ...piece.plane.yAxis, z: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: p.guid, diff: { plane: { ...p.plane!, yAxis: { ...p.plane!.yAxis, z: value } } } }));
      updatePieces(origin, updates);
    }
  };

  const commonTypeName = getCommonValue((p) => {
    const type = p.type ? findTypeInKit(kit, p.type) : null;
    return type?.name;
  });
  const commonTypeVariant = getCommonValue((p) => {
    const type = p.type ? findTypeInKit(kit, p.type) : null;
    return type?.variant;
  });
  const commonCenterX = getCommonValue((p) => p.center?.x);
  const commonCenterY = getCommonValue((p) => p.center?.y);
  const commonPlaneOriginX = getCommonValue((p) => p.plane?.origin.x);
  const commonPlaneOriginY = getCommonValue((p) => p.plane?.origin.y);
  const commonPlaneOriginZ = getCommonValue((p) => p.plane?.origin.z);
  const commonPlaneXAxisX = getCommonValue((p) => p.plane?.xAxis.x);
  const commonPlaneXAxisY = getCommonValue((p) => p.plane?.xAxis.y);
  const commonPlaneXAxisZ = getCommonValue((p) => p.plane?.xAxis.z);
  const commonPlaneYAxisX = getCommonValue((p) => p.plane?.yAxis.x);
  const commonPlaneYAxisY = getCommonValue((p) => p.plane?.yAxis.y);
  const commonPlaneYAxisZ = getCommonValue((p) => p.plane?.yAxis.z);

  // Only show plane/center for fixed pieces (pieces that have both plane and center)
  // Linked pieces (without plane) get their position from flatten algorithm
  const hasCenter = pieces.every((p) => p.center);
  const hasPlane = pieces.every((p) => p.plane);
  const hasVariant = pieces.some((p) => {
    const type = p.type ? findTypeInKit(kit, p.type) : null;
    return type?.variant;
  });
  const hasUnfixedPieces = pieces.some((p) => !p.plane || !p.center);

  const pieceIds = useMemo(() => pieces.map((p) => p.guid), [pieces]);

  const selectedVariants = useMemo(
    () => [
      ...new Set(
        pieces
          .map((p) => {
            const type = p.type ? findTypeInKit(kit, p.type) : null;
            return type?.variant;
          })
          .filter((v): v is string => Boolean(v)),
      ),
    ],
    [pieces, kit],
  );
  const availableTypes = useReplacableTypes(pieceIds, isDesignPiece ? [] : selectedVariants);
  const availableTypeNames = useMemo(() => [...new Set(availableTypes.map((t) => t.name))], [availableTypes]);
  const allReplacableTypes = useReplacableTypes(pieceIds, []);
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

  const replacableDesignsRaw = useReplacableDesigns(isSingle && piece ? piece : ({} as Piece));
  const availableDesigns = isDesignPiece && isSingle && piece ? replacableDesignsRaw : [];
  const availableDesignNames = useMemo(() => [...new Set(availableDesigns.map((d) => d.name))], [availableDesigns]);

  // Get piece type/design for pieces
  const pieceType = piece?.type && typeof piece.type === "string" && piece.type !== "design" ? findTypeInKit(kit, piece.type) : null;
  const pieceDesign = piece?.design && typeof piece.design === "string" ? findDesignInKit(kit, piece.design) : null;

  // Get available design variants and views
  const availableDesignVariants = pieceDesign
    ? [
        ...new Set(
          availableDesigns
            .filter((d) => d.name === pieceDesign.name)
            .map((d) => d.variant)
            .filter((v): v is string => Boolean(v)),
        ),
      ]
    : [];

  const availableDesignViews = pieceDesign
    ? [
        ...new Set(
          availableDesigns
            .filter((d) => d.name === pieceDesign.name && (d.variant || "") === (pieceDesign.variant || ""))
            .map((d) => d.view)
            .filter((v): v is string => Boolean(v)),
        ),
      ]
    : [];

  let parentConnection: Connection | null = null;
  let parentConnections: Connection[] = [];

  // TODO: Re-implement parent connection finding once metadata is available
  // if (isSingle && piece) {
  //   const pieceMetadata = metadata.get(getPieceId(piece));
  //   if (pieceMetadata?.parentPieceId && pieceMetadata?.parentConnectionId) {
  //     try {
  //       parentConnection = findConnectionInDesign(design, pieceMetadata.parentConnectionId);
  //     } catch {}
  //   }
  // } else if (!isSingle) {
  //   // For multiple pieces, find all their parent connections
  //   parentConnections = pieces
  //     .map((piece) => {
  //       const pieceMetadata = metadata.get(getPieceId(piece));
  //       if (pieceMetadata?.parentPieceId && pieceMetadata?.parentConnectionId) {
  //         try {
  //           return findConnectionInDesign(design, pieceMetadata.parentConnectionId);
  //         } catch {
  //           return null;
  //         }
  //       }
  //       return null;
  //     })
  //     .filter((conn) => conn !== null) as Connection[];
  // }

  return (
    <>
      {hasMixedTypes ? (
        <TreeItem>
          <TreeContent>
            <p className="text-sm text-muted-foreground">{t("semio.sketchpad.app.design.piece.mixedSelectionMessage")}</p>
          </TreeContent>
        </TreeItem>
      ) : (
        <>
          {isSingle && piece && (
            <TreeItem>
              <TreeContent>
                <Input label={t("semio.sketchpad.app.design.piece.id")} value={getPieceId(piece)} disabled />
              </TreeContent>
            </TreeItem>
          )}

          {isDesignPiece ? (
            // Design piece fields
            <>
              <TreeItem>
                <TreeContent>
                  <Combobox
                    label={t("semio.sketchpad.app.design.name")}
                    options={availableDesignNames.map((name) => ({
                      value: name,
                      label: name,
                    }))}
                    value={pieceDesign?.name || pieceType?.name || ""}
                    placeholder={t("semio.sketchpad.common.selectDesign")}
                    onValueChange={handleDesignNameChange}
                  />
                </TreeContent>
              </TreeItem>
              {availableDesignVariants.length > 0 && (
                <TreeItem>
                  <TreeContent>
                    <Combobox
                      label={t("semio.sketchpad.app.design.variant")}
                      options={availableDesignVariants.map((variant) => ({
                        value: variant,
                        label: variant,
                      }))}
                      value={pieceDesign?.variant || pieceType?.variant || ""}
                      placeholder={t("semio.sketchpad.common.selectVariant")}
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
                      label={t("semio.sketchpad.app.design.view")}
                      options={availableDesignViews.map((view) => ({
                        value: view,
                        label: view,
                      }))}
                      value={pieceDesign?.view || ""}
                      placeholder={t("semio.sketchpad.common.selectView")}
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
                    label={t("semio.sketchpad.app.design.piece.type")}
                    options={availableTypeNames.map((name) => ({
                      value: name,
                      label: name,
                    }))}
                    value={isSingle && piece && piece.type ? findTypeInKit(kit, piece.type)?.name || "" : commonTypeName || ""}
                    placeholder={!isSingle && commonTypeName === undefined ? t("semio.sketchpad.common.mixedValues") : t("semio.sketchpad.common.selectType")}
                    onValueChange={handleTypeNameChange}
                  />
                </TreeContent>
              </TreeItem>
              {(hasVariant || availableVariants.length > 0) && (
                <TreeItem>
                  <TreeContent>
                    <Combobox
                      label={t("semio.sketchpad.app.type.variant")}
                      options={availableVariants.map((variant) => ({
                        value: variant,
                        label: variant,
                      }))}
                      value={isSingle && piece && piece.type ? findTypeInKit(kit, piece.type)?.variant || "" : commonTypeVariant || ""}
                      placeholder={!isSingle && commonTypeVariant === undefined ? t("semio.sketchpad.common.mixedValues") : t("semio.sketchpad.common.selectVariant")}
                      onValueChange={handleTypeVariantChange}
                      allowClear={true}
                    />
                  </TreeContent>
                </TreeItem>
              )}
            </>
          )}
        </>
      )}
      {hasCenter && (
        <TreeItem label={t("semio.sketchpad.app.design.piece.center")}>
          <TreeItem>
            <TreeContent>
              <Stepper
                id="semio.sketchpad.app.design.panel.details.section.piece.center.x"
                value={isSingle && piece ? piece.center?.x : commonCenterX}
                onChange={handleCenterXChange}
                startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.center.x")}
                finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.center.x")}
                abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.center.x")}
                step={0.1}
              />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <Stepper
                id="semio.sketchpad.app.design.panel.details.section.piece.center.y"
                value={isSingle && piece ? piece.center?.y : commonCenterY}
                onChange={handleCenterYChange}
                startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.center.y")}
                finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.center.y")}
                abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.center.y")}
                step={0.1}
              />
            </TreeContent>
          </TreeItem>
        </TreeItem>
      )}
      {isSingle && piece && !piece.plane && (
        <TreeItem>
          <TreeContent>
            <div className="flex flex-col gap-2">
              <p className="text-sm text-muted-foreground">
                {t("semio.sketchpad.app.design.piece.connectedPieceInfo")}
              </p>
              <Button
                variant="secondary"
                onClick={() => {
                  const origin = "semio.sketchpad.app.design.panel.details.section.piece.fixPiece";
                  // TODO: Implement fix piece by getting flat plane and center, removing connection, and setting plane/center
                  console.log("[ORIGIN] Fix piece not yet implemented", origin);
                }}
              >
                <Link2Off className="h-4 w-4" />
                {t("semio.sketchpad.app.design.piece.fixPiece")}
              </Button>
            </div>
          </TreeContent>
        </TreeItem>
      )}
      {hasPlane && (
        <TreeItem label={t("semio.sketchpad.app.design.piece.plane")}>
          <TreeItem label={t("semio.sketchpad.app.design.piece.planeOrigin")}>
            <TreeItem>
              <TreeContent>
                <Stepper
                  id="semio.sketchpad.app.design.panel.details.section.piece.plane.origin.x"
                  value={isSingle && piece ? piece.plane?.origin.x : commonPlaneOriginX}
                  onChange={handlePlaneOriginXChange}
                  startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.origin.x")}
                  finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.origin.x")}
                  abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.origin.x")}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper
                  id="semio.sketchpad.app.design.panel.details.section.piece.plane.origin.y"
                  value={isSingle && piece ? piece.plane?.origin.y : commonPlaneOriginY}
                  onChange={handlePlaneOriginYChange}
                  startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.origin.y")}
                  finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.origin.y")}
                  abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.origin.y")}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper
                  id="semio.sketchpad.app.design.panel.details.section.piece.plane.origin.z"
                  value={isSingle && piece ? piece.plane?.origin.z : commonPlaneOriginZ}
                  onChange={handlePlaneOriginZChange}
                  startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.origin.z")}
                  finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.origin.z")}
                  abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.origin.z")}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
          </TreeItem>
          <TreeItem label={t("semio.sketchpad.app.design.piece.planeXAxis")}>
            <TreeItem>
              <TreeContent>
                <Stepper
                  id="semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.x"
                  value={isSingle && piece ? piece.plane?.xAxis.x : commonPlaneXAxisX}
                  onChange={handlePlaneXAxisXChange}
                  startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.x")}
                  finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.x")}
                  abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.x")}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper
                  id="semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.y"
                  value={isSingle && piece ? piece.plane?.xAxis.y : commonPlaneXAxisY}
                  onChange={handlePlaneXAxisYChange}
                  startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.y")}
                  finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.y")}
                  abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.y")}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper
                  id="semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.z"
                  value={isSingle && piece ? piece.plane?.xAxis.z : commonPlaneXAxisZ}
                  onChange={handlePlaneXAxisZChange}
                  startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.z")}
                  finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.z")}
                  abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.z")}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
          </TreeItem>
          <TreeItem label={t("semio.sketchpad.app.design.piece.planeYAxis")}>
            <TreeItem>
              <TreeContent>
                <Stepper
                  id="semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.x"
                  value={isSingle && piece ? piece.plane?.yAxis.x : commonPlaneYAxisX}
                  onChange={handlePlaneYAxisXChange}
                  startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.x")}
                  finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.x")}
                  abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.x")}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper
                  id="semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.y"
                  value={isSingle && piece ? piece.plane?.yAxis.y : commonPlaneYAxisY}
                  onChange={handlePlaneYAxisYChange}
                  startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.y")}
                  finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.y")}
                  abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.y")}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper
                  id="semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.z"
                  value={isSingle && piece ? piece.plane?.yAxis.z : commonPlaneYAxisZ}
                  onChange={handlePlaneYAxisZChange}
                  startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.z")}
                  finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.z")}
                  abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.z")}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
          </TreeItem>
        </TreeItem>
      )}
      {(parentConnection || parentConnections.length > 0) && (
        <div style={{ marginTop: "0.5rem" }}>
          <ConnectionsSection connections={isSingle && parentConnection ? [parentConnection] : parentConnections} isSingle={isSingle} count={parentConnections.length} />
        </div>
      )}
    </>
  );
};

export const ConnectionsSection: FC<{
  connections: any[];
  isSingle: boolean;
  count: number;
}> = ({ connections, isSingle, count }) => {
  const isInDesignScope = useIsInDesignScope();
  const { t } = useTranslation();
  if (!isInDesignScope) return null;
  const sectionLabel = isSingle ? t("semio.sketchpad.app.design.panel.details.parentConnection") : t("semio.sketchpad.app.design.panel.details.parentConnections", { count });
  return <ConnectionsSectionForm connections={connections} sectionLabel={sectionLabel} />;
};

const ConnectionsSectionForm: FC<{
  connections: Connection[];
  sectionLabel?: string;
}> = ({ connections, sectionLabel }) => {
  const { t } = useTranslation();
  const { updateConnection, startTransaction, finalizeTransaction, abortTransaction } = useDesignAppCommands();
  const connectionObjects = connections;

  const isSingle = connections.length === 1;
  const connection = isSingle ? connectionObjects[0] : null;

  const getCommonValue = <T,>(getter: (connection: Connection) => T | undefined): T | undefined => {
    const values = connectionObjects.map(getter).filter((v) => v !== undefined);
    if (values.length === 0) return undefined;
    const firstValue = values[0];
    return values.every((v) => JSON.stringify(v) === JSON.stringify(firstValue)) ? firstValue : undefined;
  };

  const handleChange = (updatedConnection: Connection) => {
    if (!updatedConnection || !updatedConnection.guid) return;
    const origin = "semio.sketchpad.app.design.panel.details.section.connection.change";
    
    // Calculate diff between current and updated connection
    const diff: ConnectionDiff = {};
    if (connection) {
      if (updatedConnection.gap !== connection.gap) diff.gap = updatedConnection.gap;
      if (updatedConnection.shift !== connection.shift) diff.shift = updatedConnection.shift;
      if (updatedConnection.rise !== connection.rise) diff.rise = updatedConnection.rise;
      if (updatedConnection.rotation !== connection.rotation) diff.rotation = updatedConnection.rotation;
      if (updatedConnection.turn !== connection.turn) diff.turn = updatedConnection.turn;
      if (updatedConnection.tilt !== connection.tilt) diff.tilt = updatedConnection.tilt;
      if (updatedConnection.x !== connection.x) diff.x = updatedConnection.x;
      if (updatedConnection.y !== connection.y) diff.y = updatedConnection.y;
    }
    
    updateConnection(origin, updatedConnection.guid, diff);
  };

  const handleGapChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, gap: value });
  };

  const handleShiftChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, shift: value });
  };

  const handleRiseChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, rise: value });
  };

  const handleXOffsetChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, x: value });
  };

  const handleYOffsetChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, y: value });
  };

  const handleRotationChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, rotation: value });
  };

  const handleTurnChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, turn: value });
  };

  const handleTiltChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, tilt: value });
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
    <>
      {isSingle && (
        <>
          <TreeItem>
            <TreeContent>
              <Input id="semio.sketchpad.app.design.panel.details.section.connection.connectingPieceId" value={connection!.connecting.piece} disabled showLabel />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <Input id="semio.sketchpad.app.design.panel.details.section.connection.connectingPortId" value={connection!.connecting.port} disabled showLabel />
            </TreeContent>
          </TreeItem>
          {connection!.connecting.designPiece && (
            <TreeItem>
              <TreeContent>
                <Input id="semio.sketchpad.app.design.panel.details.section.connection.connectingDesignPieceId" value={connection!.connecting.designPiece} disabled showLabel />
              </TreeContent>
            </TreeItem>
          )}
          <TreeItem>
            <TreeContent>
              <Input id="semio.sketchpad.app.design.panel.details.section.connection.connectedPieceId" value={connection!.connected.piece} disabled showLabel />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <Input id="semio.sketchpad.app.design.panel.details.section.connection.connectedPortId" value={connection!.connected.port} disabled showLabel />
            </TreeContent>
          </TreeItem>
          {connection!.connected.designPiece && (
            <TreeItem>
              <TreeContent>
                <Input id="semio.sketchpad.app.design.panel.details.section.connection.connectedDesignPieceId" value={connection!.connected.designPiece} disabled showLabel />
              </TreeContent>
            </TreeItem>
          )}
        </>
      )}
      {!isSingle && (
        <TreeItem>
          <TreeContent>
            <p className="text-sm text-muted-foreground">{t("semio.sketchpad.app.design.panel.details.section.connection.multipleEditing", { count: connections.length })}</p>
          </TreeContent>
        </TreeItem>
      )}
      <TreeItem>
        <TreeContent>
          <Stepper
            id="semio.sketchpad.app.design.panel.details.section.connection.gap"
            value={isSingle ? (connection!.gap ?? 0) : (commonGap ?? 0)}
            onChange={handleGapChange}
            startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.gap")}
            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.gap")}
            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.gap")}
            step={0.1}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Stepper
            id="semio.sketchpad.app.design.panel.details.section.connection.shift"
            value={isSingle ? (connection!.shift ?? 0) : (commonShift ?? 0)}
            onChange={handleShiftChange}
            startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.shift")}
            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.shift")}
            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.shift")}
            step={0.1}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Stepper
            id="semio.sketchpad.app.design.panel.details.section.connection.rise"
            value={isSingle ? (connection!.rise ?? 0) : (commonRise ?? 0)}
            onChange={handleRiseChange}
            startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.rise")}
            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.rise")}
            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.rise")}
            step={0.1}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <div className="flex flex-col gap-1">
            <label className="text-xs">{t("semio.sketchpad.app.design.connection.rotation")}</label>
            <Slider
              id="semio.sketchpad.app.design.panel.details.section.connection.rotation"
              value={[isSingle ? (connection!.rotation ?? 0) : (commonRotation ?? 0)]}
              onValueChange={([value]) => handleRotationChange(value)}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.rotation")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.rotation")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.rotation")}
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
            <label className="text-xs">{t("semio.sketchpad.app.design.connection.turn")}</label>
            <Slider
              id="semio.sketchpad.app.design.panel.details.section.connection.turn"
              value={[isSingle ? (connection!.turn ?? 0) : (commonTurn ?? 0)]}
              onValueChange={([value]) => handleTurnChange(value)}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.turn")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.turn")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.turn")}
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
            <label className="text-xs">{t("semio.sketchpad.app.design.connection.tilt")}</label>
            <Slider
              id="semio.sketchpad.app.design.panel.details.section.connection.tilt"
              value={[isSingle ? (connection!.tilt ?? 0) : (commonTilt ?? 0)]}
              onValueChange={([value]) => handleTiltChange(value)}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.tilt")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.tilt")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.tilt")}
              min={-180}
              max={180}
              step={1}
            />
          </div>
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Stepper
            id="semio.sketchpad.app.design.panel.details.section.connection.x"
            value={isSingle ? (connection!.x ?? 0) : (commonXOffset ?? 0)}
            onChange={handleXOffsetChange}
            startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.x")}
            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.x")}
            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.x")}
            step={0.1}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Stepper
            id="semio.sketchpad.app.design.panel.details.section.connection.y"
            value={isSingle ? (connection!.y ?? 0) : (commonYOffset ?? 0)}
            onChange={handleYOffsetChange}
            startTransaction={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.y")}
            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.y")}
            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.y")}
            step={0.1}
          />
        </TreeContent>
      </TreeItem>
    </>
  );
};

export const PortSection: FC<{ pieceGuid: Guid; portGuid: Guid }> = ({ pieceGuid, portGuid }) => {
  const isInDesignScope = useIsInDesignScope();
  if (!isInDesignScope) return null;
  return <PortSectionForm pieceGuid={pieceGuid} portGuid={portGuid} />;
};

const PortSectionForm: FC<{ pieceGuid: Guid; portGuid: Guid }> = ({ pieceGuid, portGuid }) => {
  const { t } = useTranslation();
  const design = useDesign() as Design;
  const kit = useKit() as Kit;

  const piece = (() => {
    try {
      return findPieceInDesign(design, pieceGuid);
    } catch {
      return null;
    }
  })();

  const type = piece?.type && typeof piece.type === "string" ? findTypeInKit(kit, piece.type) : null;
  const port = type?.ports?.find((p) => p.guid === portGuid);

  if (!piece || !type || !port) {
    return (
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{t("semio.sketchpad.app.design.panel.details.section.port.notFound")}</p>
        </TreeContent>
      </TreeItem>
    );
  }

  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.design.panel.details.section.port.id" value={port.guid || "~default~"} disabled showLabel />
        </TreeContent>
      </TreeItem>
      {port.description && (
        <TreeItem>
          <TreeContent>
            <Textarea id="semio.sketchpad.app.design.panel.details.section.port.description" value={port.description} disabled showLabel />
          </TreeContent>
        </TreeItem>
      )}
      {port.family && (
        <TreeItem>
          <TreeContent>
            <Input id="semio.sketchpad.app.design.panel.details.section.port.family" value={port.family} disabled showLabel />
          </TreeContent>
        </TreeItem>
      )}
      {port.mandatory !== undefined && (
        <TreeItem>
          <TreeContent>
            <Input id="semio.sketchpad.app.design.panel.details.section.port.mandatory" value={port.mandatory ? t("semio.sketchpad.common.yes") : t("semio.sketchpad.common.no")} disabled showLabel />
          </TreeContent>
        </TreeItem>
      )}
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.design.panel.details.section.port.position" value={`(${port.point.x.toFixed(2)}, ${port.point.y.toFixed(2)}, ${port.point.z.toFixed(2)})`} disabled showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.design.panel.details.section.port.direction" value={`(${port.direction.x.toFixed(2)}, ${port.direction.y.toFixed(2)}, ${port.direction.z.toFixed(2)})`} disabled showLabel />
        </TreeContent>
      </TreeItem>
      {port.compatibleFamilies &&
        port.compatibleFamilies.map((family: string, index: number) => (
          <TreeItem key={`compatible-family-${index}`}>
            <TreeContent>
              <Input id="semio.sketchpad.app.design.panel.details.section.port.compatibleFamily" value={family} disabled showLabel />
            </TreeContent>
          </TreeItem>
        ))}
      {port.attributes &&
        port.attributes.map((attribute: any, index: number) => (
          <TreeItem key={`port-attribute-${index}`}>
            <TreeContent>
              <Input id="semio.sketchpad.app.design.panel.details.section.port.attribute" value={`${attribute.key}: ${attribute.value || "N/A"} ${attribute.unit && `(${attribute.unit})`}`} disabled showLabel />
            </TreeContent>
          </TreeItem>
        ))}
    </>
  );
};
