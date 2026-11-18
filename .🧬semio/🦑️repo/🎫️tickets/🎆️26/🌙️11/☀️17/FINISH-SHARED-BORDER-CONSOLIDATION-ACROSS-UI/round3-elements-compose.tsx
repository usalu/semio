import * as React from "react";
import { Action, ActionGroup, ActionGroupItem, Button, ButtonGroup, ButtonGroupItem, Checkbox, Field, Input, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, SharedBorderRow, SharedBorderRowCell, SharedBorderRowField, SharedBorderRowLabel, Textarea, ToggleGroup } from "@semio-tech/ui-react";

const items = [{ value: "a", id: "r3-tg-a", icon: "box" as const, text: "Alpha" }, { value: "b", id: "r3-tg-b", icon: "list" as const, text: "Beta" }, { value: "c", id: "r3-tg-c", icon: "x" as const, text: "Gamma", disabled: true }];

/** 🧪️ Representative compositions of the element sweep: forms, groups, wrapping rows, and the idle/disabled/invalid/destructive states. */
export const compositions: Record<string, () => React.ReactNode> = {
  Form: () => (
    <div className="flex w-[28rem] flex-col">
      <Field id="r3-f-name" label="Name"><Input id="r3-f-name" placeholder="Kisho" /></Field>
      <Field id="r3-f-mail" label="Email" error="Enter a valid email address."><Input id="r3-f-mail" aria-invalid defaultValue="nope" /></Field>
      <Field id="r3-f-off" label="Disabled"><Input id="r3-f-off" disabled defaultValue="locked" /></Field>
      <Field id="r3-f-note" label="Notes"><Textarea id="r3-f-note" /></Field>
    </div>
  ),
  Groups: () => (
    <div className="flex flex-col items-start gap-[40px]">
      <ToggleGroup kind="single" defaultValue="a" items={items} />
      <ButtonGroup><ButtonGroupItem icon="box" text="One" /><ButtonGroupItem icon="list" text="Two" disabled /><ButtonGroupItem icon="x" text="Three" /></ButtonGroup>
      <ActionGroup id="r3-ag"><ActionGroupItem id="r3-ag-1" icon="box" text="Act" /><ActionGroupItem id="r3-ag-2" icon="x" text="Off" disabled /></ActionGroup>
      <Button icon="save" text="Save" />
      <Button icon="x" text="Disabled" disabled />
    </div>
  ),
  Rows: () => (
    <div className="flex flex-col">
      <SharedBorderRow wrap className="w-[18rem]">{Array.from({ length: 9 }, (_, i) => <SharedBorderRowCell key={i}>Cell {i}</SharedBorderRowCell>)}</SharedBorderRow>
      <SharedBorderRow orientation="vertical" tone="destructive" className="w-[18rem]">
        <SharedBorderRowCell tone="destructive">Destructive</SharedBorderRowCell>
        <SharedBorderRow framed={false} data-shared-border-cell=""><SharedBorderRowLabel htmlFor="r3-row-a">Label</SharedBorderRowLabel><SharedBorderRowField><input id="r3-row-a" aria-invalid defaultValue="x" /></SharedBorderRowField></SharedBorderRow>
        <SharedBorderRow framed={false} data-shared-border-cell=""><Checkbox id="r3-row-c" /><SharedBorderRowCell>Check</SharedBorderRowCell></SharedBorderRow>
      </SharedBorderRow>
    </div>
  ),
  Select: () => (
    <Select defaultValue="a"><SelectTrigger id="r3-sel" className="w-[12rem]"><SelectValue /></SelectTrigger><SelectContent><SelectItem value="a">Alpha</SelectItem><SelectItem value="b">Beta</SelectItem></SelectContent></Select>
  ),
};
