import React from 'react'
import ReactDOM from 'react-dom/client'

import './globals.css'

import { Sketchpad } from '@semio/js'

declare global {
    interface Window {
        windowControls: {
            minimize(): Promise<any>;
            maximize(): Promise<any>;
            close(): Promise<any>;
        }
    }
}

const invokeWindowControl = (action: 'minimize' | 'maximize' | 'close') => {
    if (window.windowControls) {
        return window.windowControls[action]();
    }
    console.warn(`Window controls not available for action: ${action}`);
    return Promise.resolve();
};

const windowEvents = {
    minimize: () => invokeWindowControl('minimize'),
    maximize: () => invokeWindowControl('maximize'),
    close: () => invokeWindowControl('close')
};

function App() {
    return (
        <div className="h-screen w-screen">
            <Sketchpad onWindowEvents={windowEvents} />
        </div>
    );
}

export default App;

ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
)