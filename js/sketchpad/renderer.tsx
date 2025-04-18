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
        os: {
            getUserId(): Promise<string>;
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

const invokeOs = (action: 'get-user-id') => {
    if (window.os) {
        return window.os[action]();
    }
    console.warn(`OS not available for action: ${action}`);
    return Promise.resolve();
};

const os = {
    getUserId: () => invokeOs('get-user-id')
};

function App() {
    console.log(os.getUserId());
    return (
        <div className="h-screen w-screen">
            <Sketchpad onWindowEvents={windowEvents} userId={os.getUserId()} />
        </div>
    );
}

export default App;

ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
)