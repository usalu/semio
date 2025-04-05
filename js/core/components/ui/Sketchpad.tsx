import React, { FC, Suspense } from 'react';

import {
    ResizableHandle,
    ResizablePanel,
    ResizablePanelGroup,
} from "@semio/js/components/ui/Resizable"
import { Diagram } from '@semio/js';

interface SketchpadProps {
}
const Sketchpad: FC<SketchpadProps> = ({ }) => {
    return (
        <div className="h-[800px] w-[1300px]">

            <ResizablePanelGroup
                direction="horizontal"
                className="bg-dark text-light"
            >
                <ResizablePanel defaultSize={300}>
                    <div className="flex h-full items-center justify-center p-6">
                        Tool
                    </div>
                </ResizablePanel>
                <ResizableHandle />
                <ResizablePanel defaultSize={800}>
                    <ResizablePanelGroup direction="vertical" className="border-l border-r">
                        <ResizablePanel defaultSize={400}>
                            <ResizablePanelGroup direction="horizontal" className="border-b">
                                <ResizablePanel defaultSize={400} className="border-r">
                                    <Diagram fullscreen={false} />
                                </ResizablePanel>
                                <ResizableHandle />
                                <ResizablePanel defaultSize={400}>
                                    <div className="flex h-full items-center justify-center p-6">
                                        <span className="font-semibold">Three</span>
                                    </div>
                                </ResizablePanel>
                            </ResizablePanelGroup>
                        </ResizablePanel>
                        <ResizableHandle />
                        <ResizablePanel defaultSize={200}>
                            <div className="flex h-full items-center justify-center p-6">
                                Console
                            </div>
                        </ResizablePanel>
                    </ResizablePanelGroup>
                </ResizablePanel>
                <ResizableHandle />
                <ResizablePanel defaultSize={200}>
                    <div className="flex h-full items-center justify-center p-6">
                        <span className="font-semibold">Details</span>
                    </div>
                </ResizablePanel>
            </ResizablePanelGroup>
        </div>
    );
};

export default Sketchpad;