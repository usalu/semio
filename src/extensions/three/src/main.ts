import {Server,ServerUnaryCall,sendUnaryData,ServerCredentials,ChannelCredentials} from '@grpc/grpc-js';
import {GrpcTransport} from "@protobuf-ts/grpc-transport";
import {Representation} from 'semio/model/v1/model'
import {RepresentationConversionRequest} from 'semio/extension/converter/v1/converter'
import {IConverterService,converterServiceDefinition} from 'semio/extension/converter/v1/converter.grpc-server'
import {RegisterExtensionRequest} from 'semio/manager/v1/manager'
import {ManagerServiceClient} from 'semio/manager/v1/manager.client'
import {THREE as SEMIO_THREE,RHINO} from 'semio/constants'
import * as THREE from 'three'
import {Rhino3dmLoader} from 'three/examples/jsm/loaders/3DMLoader'
import fs from 'fs';
import tmp from 'tmp';
tmp.setGracefulCleanup();


const name = "semio.three"
const address = '[::]:' + SEMIO_THREE['PORT'];

const threeConverterService: IConverterService = {

    convertRepresentation(call: ServerUnaryCall<RepresentationConversionRequest, Representation>, callback: sendUnaryData<Representation>): void {
      switch (call.request.targetType){
        case RHINO['FILEEXTENSION']:
            const tempRhinoFileName = tmp.tmpNameSync();
            fs.writeFile(tempRhinoFileName, new TextDecoder().decode(call.request.representation?.body?.value),err =>{console.log(err);
            });
            
            const loader = new Rhino3dmLoader();
            loader.setLibraryPath( 'https://cdn.jsdelivr.net/npm/rhino3dm@0.15.0-beta/' );
            loader.load(
                tempRhinoFileName,
                rhinoThreeObject => { 
                    return {
                        body: {
                            value: rhinoThreeObject.toJSON()
                        }
                    }
                },
                xhr => {},
                error => { console.log(error);}
            );
            break;

      }
        console.log("Converting representation...")
      callback(null,{name:"HelloRep", lod:BigInt(435)});
    }
}


function getServer(): Server {
    const server = new Server();
    server.addService(converterServiceDefinition, threeConverterService);
    return server;
}

// function convertRepresentation(call : grpc.ServerUnaryCall<RepresentationConversionRequest, Representation>,callback: grpc.sendUnaryData<Representation>){
//     console.log("Converting representation...")
//     callback(null,{name:"HelloRep", lod:BigInt(435)})
// }

// function getServer(): grpc.Server {
//     const server = new grpc.Server();
//     server.addService(converterServiceDefinition, {
//         "convertRepresentation": convertRepresentation
//     });
//     return server;
// }

// function delay(ms: number) {
//     return new Promise( resolve => setTimeout(resolve, ms) );
// }

async function registerExtension() {
    const client = new ManagerServiceClient(new GrpcTransport({ host: "localhost:50002", channelCredentials: ChannelCredentials.createInsecure()}))
    const registration = client.registerExtension({
        replaceExisting: true,
        name: name,
        address: address,
        extending:{
            adaptings: [],
            convertings:[
                {
                    sourceTypeUrl:"mcneel/rhino",
                    targetTypeUrl:"mrdoob/three"
                }
            ],
            transformings : [],
            translatings : []
        }})
    const response = await registration.response
    // if (response.success===false){
    //     console.log("Attempt to register extension failed. Will try again in 2s.")
    //     await delay(4000).then(() => { registerExtension()})
    // }
    console.log(response)
}


if (require.main === module) {
    const server = getServer();
    server.bindAsync(
        address,
        ServerCredentials.createInsecure(),
        (err: Error | null, port: number) => {
            if (err) {
                console.error(`Server error: ${err.message}`);
            } else {
                console.log(`Server bound on port: ${port}`);
                registerExtension()
                server.start();
            }
        }
    );
}

