import { FC, useState } from "react"

// store.tsx



// commands/**
const CreatePieceArray: FC = () => {
    //transaction, createPiece, createConnection
    const [count, setCount] = useState(10);
    return (
        <Slider max={100} min={0} step={1} onChange={setCount(value)} />
        <button onClick={() => {
            transaction(() => {
                createPiece();
                createConnection();
            });
        }}>Create Piece</button>
    )
}

// DesignEditor.tsx
const DesignEditor: FC = () => {
    // design, undo, redo, updateSelection

    return (
        <div>
            <h1>{design.name}</h1>
            <button onClick={undo}>Undo</button>
            <button onClick={redo}>Redo</button>
            <Diagram design={design} onSelect={updateSelection} />
            <Model design={design onSelect={updateSelection}}
        </div>
    );
};


// Sketchpad.tsx
const Sketchpad: FC<SketchpadProps> = () => {
    // undo, redo, createKit, createDesign, designs
    return (
        <Studio>
            <button onClick={undo}>Undo</button>
            <button onClick={redo}>Redo</button>
            <DesignEditor />
        </Studio>
    );
};
