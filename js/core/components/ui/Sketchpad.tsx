import React, { FC, Suspense } from 'react';

import {
    ResizableHandle,
    ResizablePanel,
    ResizablePanelGroup,
} from "@semio/js/components/ui/Resizable"

interface SketchpadProps {
}
const Sketchpad: FC<SketchpadProps> = ({ }) => {
    return (
        <div className="h-full w-full">

            <ResizablePanelGroup
                direction="horizontal"
                className="max-w-md rounded-lg border md:min-w-[450px]"
            >
                <ResizablePanel defaultSize={75}>
                    <div className="flex h-full items-center justify-center p-6">
                        <span className="font-semibold">Tool</span>
                    </div>
                </ResizablePanel>
                <ResizableHandle />
                <ResizablePanel defaultSize={75}>
                    <ResizablePanelGroup direction="vertical">
                        <ResizablePanel defaultSize={75}>
                            <ResizablePanelGroup direction="horizontal">
                                <ResizablePanel defaultSize={25}>
                                    <div className="flex h-full items-center justify-center p-6">
                                        <span className="font-semibold">Diagram</span>
                                    </div>
                                </ResizablePanel>
                                <ResizableHandle />
                                <ResizablePanel defaultSize={75}>
                                    <div className="flex h-full items-center justify-center p-6">
                                        <span className="font-semibold">Three</span>
                                    </div>
                                </ResizablePanel>
                            </ResizablePanelGroup>
                        </ResizablePanel>
                        <ResizableHandle />
                        <ResizablePanel defaultSize={75}>
                            <div className="flex h-full items-center justify-center p-6">
                                <span className="font-semibold">Console</span>
                            </div>
                        </ResizablePanel>
                    </ResizablePanelGroup>
                </ResizablePanel>
                <ResizableHandle />
                <ResizablePanel defaultSize={100}>
                    <div className="flex h-full items-center justify-center p-6">
                        <span className="font-semibold">Details</span>
                    </div>
                </ResizablePanel>
            </ResizablePanelGroup>
        </div>
    );
};

export default Sketchpad;